use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import the extracted function from output2
use wrapped_split_decls_rs::process_crates_in_path;

// Define required types for the function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitDeclsConfig {
    pub patches: HashMap<String, String>,
    pub string_replacements: HashMap<String, String>,
    pub custom_prelude_overlay: Option<String>,
}

impl Default for SplitDeclsConfig {
    fn default() -> Self {
        Self {
            patches: HashMap::new(),
            string_replacements: HashMap::new(),
            custom_prelude_overlay: None,
        }
    }
}

pub fn test_process_crates_function() -> Result<()> {
    println!("Testing extracted process_crates_in_path function...");
    
    let test_path = Path::new(".");
    let current_crate = "bootstrap3";
    let config = SplitDeclsConfig::default();
    
    // Call the extracted function in dry-run mode
    process_crates_in_path(
        test_path,
        current_crate,
        &config,
        false, // is_rustc_source
        true,  // dry_run
    )?;
    
    println!("✅ Successfully called extracted function!");
    Ok(())
}
