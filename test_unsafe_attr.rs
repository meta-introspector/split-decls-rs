// Minimal test case for unsafe attribute error

#[no_mangle]
pub fn test_function() {
    println!("Hello");
}

fn main() {
    test_function();
}
