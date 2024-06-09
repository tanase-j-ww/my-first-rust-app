use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut input = String::new();

    io::stdin()
     .read_line(&mut input)
     .expect("failed to read input");

    let input: u32 = input.trim().parse().expect("failed to parse input");

    println!("your input is {}", input);
    println!("the secret number is {}", secret_number);

    match input.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you selected the correct secret number"),
    }

}
