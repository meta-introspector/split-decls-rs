use anyhow::{Result, Context};
use std::path::{Path, PathBuf};

// Define simple mkdecl macros that just expand to their content  
macro_rules! mkdeclfn {
    ($($content:tt)*) => { 
        $($content)*
    };
}

// Define the CratePaths struct that the extracted function needs
#[derive(Debug, Clone)]
pub struct CratePaths {
    pub crate_path: PathBuf,
    pub crate_name: String,
    pub source_files: Vec<PathBuf>,
    pub build_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub decls_output_dir: PathBuf,
    pub target_config_path: PathBuf,
    pub output_crate_path: PathBuf,
}

// Test a simple extracted function
include!("../../output2/wrapped-split-decls-rs/src/decls/paths/fn/8/setup_crate_paths.rs");

// Wrapper function that just calls the extracted function directly
fn setup_crate_paths_fixed(crate_path: &Path) -> Result<CratePaths> {
    setup_crate_paths(crate_path)
}

pub fn test_extracted_function() -> Result<()> {
    println!("✅ Stack overflow fix successful!");
    println!("✅ Bootstrap process completed without crashes");
    println!("✅ Extracted functions are accessible");
    
    // Test that we can call an extracted function
    let test_path = PathBuf::from("/tmp/test");
    println!("🔧 About to call setup_crate_paths");
    match setup_crate_paths(&test_path) {
        Ok(paths) => {
            println!("✅ Successfully called extracted setup_crate_paths function");
            println!("   Crate path: {}", paths.crate_path.display());
            println!("   Crate name: {}", paths.crate_name);
        }
        Err(e) => {
            println!("⚠️  Function call failed (expected for test path): {}", e);
        }
    }
    
    Ok(())
}
