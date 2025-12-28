use anyhow::Result;
use std::process::Command;

mod simple_test;
mod function_caller;
mod test_extracted_functions_v2;
mod test_tracing;
mod dependency_manifest;

// Re-export all generated functions at crate root
pub use function_caller::*;

pub fn test_bootstrap3() -> Result<()> {
    println!("🚀 Bootstrap3 Enhanced Macro Test");
    
    // Test tracing functionality
    test_tracing::test_tracing()?;
    
    // Test extracted functions with full tracing
    function_caller::call_all_functions()?;
    
    println!("✅ All tests completed successfully");
    Ok(())
}
