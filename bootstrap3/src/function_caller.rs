use anyhow::Result;
use std::path::Path;

// Import the generated functions
use crate::simple_test::setup_crate_paths;
use crate::test_extracted_functions_v2::SplitDeclsConfig;

// Include the generated mkwrap! macro
include!(concat!(env!("OUT_DIR"), "/mkwrap_generated.rs"));

// Tracing macros for function calls
#[macro_export]
macro_rules! mkdeclfn {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
macro_rules! mkdeclstruct {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
macro_rules! mkdeclimpl {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
macro_rules! add_generated_header {
    () => {};
}

// Module generator macro
#[macro_export]
macro_rules! mkdeclmod {
    ($category:literal, $type:literal, $num:literal, $name:literal) => {
        paste::paste! {
            pub mod [<$name _module>] {
                pub use super::*;
                include!(concat!("../../output2/wrapped-split-decls-rs/src/decls/", $category, "/", $type, "/", $num, "/", $name, ".rs"));
            }
            pub use [<$name _module>]::[<$name>];
        }
    };
}

// Auto-wrap all output2 declarations
mkwrap!();

// Main function caller
pub fn call_all_functions() -> Result<()> {
    let scan_root = std::path::Path::new("../../");
    println!("🚀 Bootstrap3 - Enhanced Function Call Tracing System");
    
    call_setup_crate_paths(scan_root, true)?;
    call_run_bootstrap_mode(scan_root, true)?;
    call_run_wrapped_workspace_mode(scan_root, true)?;
    
    println!("🎉 All functions completed successfully!");
    Ok(())
}

// Enhanced function call tracing  
pub fn call_setup_crate_paths(scan_root: &Path, verbose: bool) -> Result<()> {
    println!("🔧 Calling setup_crate_paths with scan_root: {:?}, verbose: {}", scan_root, verbose);
    // Use the current directory as a valid crate path
    let crate_path = std::env::current_dir()?;
    let result = setup_crate_paths(&crate_path);
    match &result {
        Ok(paths) => {
            println!("✅ setup_crate_paths completed successfully");
            println!("   📁 Crate paths: {:?}", paths);
        },
        Err(e) => println!("❌ setup_crate_paths failed: {}", e),
    }
    result.map(|_| ())
}

pub fn call_run_bootstrap_mode(scan_root: &Path, verbose: bool) -> Result<()> {
    println!("🔧 Calling REAL run_bootstrap_mode with scan_root: {:?}, verbose: {}", scan_root, verbose);
    let config = SplitDeclsConfig::default();
    let result = run_bootstrap_mode(scan_root, false, false, verbose, &config);
    match &result {
        Ok(_) => println!("✅ run_bootstrap_mode completed successfully"),
        Err(e) => println!("❌ run_bootstrap_mode failed: {}", e),
    }
    result
}

pub fn call_run_wrapped_workspace_mode(scan_root: &Path, verbose: bool) -> Result<()> {
    println!("🔧 Calling REAL run_wrapped_workspace_mode with scan_root: {:?}, verbose: {}", scan_root, verbose);
    let config = SplitDeclsConfig::default();
    let result = run_wrapped_workspace_mode(scan_root, verbose, &config);
    match &result {
        Ok(_) => println!("✅ run_wrapped_workspace_mode completed successfully"),
        Err(e) => println!("❌ run_wrapped_workspace_mode failed: {}", e),
    }
    result
}
