use std::fs;

// syscallclippy! - 8-level recursive macro system
#[macro_export]
macro_rules! syscallclippy {
    (level8, $syscall:expr) => {
        format!("#[syscall=\"{}\"]", $syscall)
    };
    (level7, $syscall:expr) => { syscallclippy!(level8, $syscall) };
    (level6, $syscall:expr) => { syscallclippy!(level7, $syscall) };
    (level5, $syscall:expr) => { syscallclippy!(level6, $syscall) };
    (level4, $syscall:expr) => { syscallclippy!(level5, $syscall) };
    (level3, $syscall:expr) => { syscallclippy!(level4, $syscall) };
    (level2, $syscall:expr) => { syscallclippy!(level3, $syscall) };
    ($syscall:expr) => { syscallclippy!(level2, $syscall) };
}

fn main() {
    // Original code
    let code = r#"fn example() {
    std::fs::write("test.txt", "data").unwrap();
    std::fs::read_to_string("test.txt").unwrap();
}"#;
    
    // Apply syscall annotations using 8-level recursive macro
    let annotated = code
        .replace("std::fs::write", &format!("{}\n    std::fs::write", syscallclippy!("write")))
        .replace("std::fs::read", &format!("{}\n    std::fs::read", syscallclippy!("read")));
    
    println!("Original code:\n{}\n", code);
    println!("Annotated code:\n{}", annotated);
    
    // Write to file
    fs::write("annotated_code.rs", &annotated).unwrap();
    println!("\n✅ 8-level recursive syscall annotations added!");
    println!("✅ File written: annotated_code.rs");
}
