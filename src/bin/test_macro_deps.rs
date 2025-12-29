use split_decls_rs::extract_dependencies_from_macro_wrapped_code;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file");
    
    let deps = extract_dependencies_from_macro_wrapped_code(&content);
    
    println!("Dependencies extracted from {}:", file_path);
    for dep in &deps {
        println!("  {}", dep);
    }
    println!("Total: {} dependencies", deps.len());
}
