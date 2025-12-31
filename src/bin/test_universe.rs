use split_decls_genesis_step4::mkrust;

fn main() {
    println!("🌌 Testing universe...");
    mkrust!();
    let compiler = create_rust_universe();
    println!("✅ Compiler: {}", compiler.version);
    println!("🎉 Universe test complete!");
}