use std::io::{self, Write};
use meval::eval_str;

fn main() {
    println!("Welcome! Enter what you want to calculate.");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut calc = String::new();
    io::stdin().read_line(&mut calc).unwrap();
    match eval_str(&calc) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("An error occurred: {}", e),
    }
}