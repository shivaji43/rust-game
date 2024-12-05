use std::{cmp::Ordering, io};


use rand::Rng;
fn main() {
    println!("Guess the number");
    let secret_number:u32 = rand::thread_rng().gen_range(1..100);
    //println!("The secret number is {secret_number}");
    
    
    loop{
        println!("Please Input your number");

    let mut guess = String::new();
    
    
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to take input");

    let guess:u32= match guess.trim().parse(){
        Ok(num)=> num,
        Err(_)=> continue,
    };
    println!("you guessed {}",guess);
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("too Big"),
        Ordering::Equal => {
            println!("correct guess");
            break;
        }
    }
    }
    
}
