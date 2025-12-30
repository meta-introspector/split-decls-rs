use std::fs;
use anyhow::Result;

// Include the generated pure macros
include!("../pure_macros.rs");

fn main() -> Result<()> {
    println!("🔍 Loading rustc dependencies with print tracking");
    
    // Import all dependencies with simple call
    mkbin!();
    
    println!("✅ All rustc dependencies loaded successfully");
    
    Ok(())
}
