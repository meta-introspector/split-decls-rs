// Build script to enable compiler bootstrap mode for rustc crates only
use std::env;

fn main() {
    // Only enable bootstrap mode for rustc crates, not external dependencies
    let crate_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    
    if crate_name.starts_with("rustc_") || crate_name == "split-decls-genesis" {
        // Enable bootstrap mode - this allows stability attributes
        println!("cargo:rustc-cfg=bootstrap");
        
        // Enable staged API for stability attributes
        println!("cargo:rustc-cfg=staged_api");
        
        // Set rustc version info (required for some compiler crates)
        println!("cargo:rustc-env=CFG_VERSION=1.85.0-dev");
        println!("cargo:rustc-env=CFG_VER_HASH=unknown");
        println!("cargo:rustc-env=CFG_VER_DATE=unknown");
    }
    
    // Set environment variables that rustc expects during bootstrap
    println!("cargo:rustc-env=CFG_COMPILER_HOST_TRIPLE={}", env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string()));
}
