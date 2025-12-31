use split_decls_genesis::*;

fn main() {
    println!("🧬 Testing compiler integration macros...");
    
    // Create the Rust universe using our mkrust! macro
    mkrust!();
    
    let compiler = create_rust_universe();
    println!("✅ Created Rust compiler: version {}", compiler.version);
    println!("✅ Features: {:?}", compiler.features);
    
    // Test compilation
    let source_code = "fn hello() { println!(\"Hello from generated code!\"); }";
    match compiler.compile(source_code) {
        Ok(result) => println!("✅ Compilation result: {}", result),
        Err(e) => println!("❌ Compilation error: {}", e),
    }
    
    println!("🎉 Compiler integration test complete!");
}
