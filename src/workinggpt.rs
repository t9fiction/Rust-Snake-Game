use crossterm::{
    cursor, event::{self, Event, KeyCode},
    execute, queue, terminal, ExecutableCommand,
};
use std::{collections::VecDeque, io::{stdout, Write}, time::{Duration, Instant}};
use std::thread::sleep;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
struct Position {
    x: u16,
    y: u16,
}

struct SnakeGame {
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    width: u16,
    height: u16,
    game_over: bool,
}

impl SnakeGame {
    fn new(width: u16, height: u16) -> Self {
        let initial_position = Position { x: width / 2, y: height / 2 };
        let snake = VecDeque::from(vec![initial_position]);
        let food = Position { x: 3, y: 3 };

        Self {
            snake,
            direction: Direction::Right,
            food,
            width,
            height,
            game_over: false,
        }
    }

    fn draw(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

        for pos in &self.snake {
            queue!(stdout, cursor::MoveTo(pos.x, pos.y), crossterm::style::Print("■")).unwrap();
        }

        queue!(
            stdout,
            cursor::MoveTo(self.food.x, self.food.y),
            crossterm::style::Print("●"),
            cursor::MoveTo(0, 0)
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                self.direction = match key_event.code {
                    KeyCode::Up if self.direction != Direction::Down => Direction::Up,
                    KeyCode::Down if self.direction != Direction::Up => Direction::Down,
                    KeyCode::Left if self.direction != Direction::Right => Direction::Left,
                    KeyCode::Right if self.direction != Direction::Left => Direction::Right,
                    _ => self.direction,
                };
            }
        }
    }

    fn update(&mut self) {
        let mut new_head = *self.snake.front().unwrap();

        new_head = match self.direction {
            Direction::Up => Position { x: new_head.x, y: new_head.y.saturating_sub(1) },
            Direction::Down => Position { x: new_head.x, y: new_head.y.saturating_add(1) },
            Direction::Left => Position { x: new_head.x.saturating_sub(1), y: new_head.y },
            Direction::Right => Position { x: new_head.x.saturating_add(1), y: new_head.y },
        };

        if new_head == self.food {
            self.food = Position {
                x: rand::random::<u16>() % self.width,
                y: rand::random::<u16>() % self.height,
            };
        } else {
            self.snake.pop_back();
        }

        if self.snake.contains(&new_head) || new_head.x >= self.width || new_head.y >= self.height {
            self.game_over = true;
        } else {
            self.snake.push_front(new_head);
        }
    }
}

fn main() {
    let (width, height) = (40, 20);
    let mut game = SnakeGame::new(width, height);

    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut last_update = Instant::now();
    while !game.game_over {
        if last_update.elapsed() >= Duration::from_millis(100) {
            game.handle_input();
            game.update();
            game.draw();
            last_update = Instant::now();
        }
        sleep(Duration::from_millis(10));
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
    println!("Game Over!");
}
