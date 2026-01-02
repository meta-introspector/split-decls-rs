#![feature(try_blocks)]

fn main() {
    // Simple try block test - the exact pattern that was breaking syn parsing
    let result: Result<i32, &str> = try {
        match Some(42) {
            Some(x) => x,
            None => 0, // Can't return from try block in main
        }
    };
    
    println!("✅ Try block compiles and works: {:?}", result);
    
    // More complex try block like in translation.rs
    let complex_result: Result<String, &str> = try {
        match Some(Ok::<String, &str>("success".to_string())) {
            Some(Ok(value)) => value,
            Some(Err(_)) => "error fallback".to_string(),
            None => "fallback".to_string(),
        }
    };
    
    println!("✅ Complex try block works: {:?}", complex_result);
}
