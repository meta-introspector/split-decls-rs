// Minimal build.rs - expensive processing moved to separate tool
// Run: cargo run --bin process_rustc_files for expensive operations

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    
    // Check if expensive processing was done
    if std::path::Path::new("build_cache.json").exists() {
        println!("cargo:rustc-cfg=feature=\"processed\"");
    } else {
        println!("cargo:warning=Run 'cargo run --bin process_rustc_files' for full processing");
    }
}
