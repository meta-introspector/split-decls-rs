use anyhow::Result;
use rayon::prelude::*;
use split_decls_rs::{SplitDeclsConfig, CratePaths, eager_split_crate};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🚀 Parallel Wrapper: Processing all TOML crates");
    
    // Create logs directory
    fs::create_dir_all("logs")?;
    
    // Load config
    let config_path = Path::new("split-decls-rs.toml");
    let config = if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        toml::from_str(&config_content)?
    } else {
        SplitDeclsConfig::default()
    };
    
    // Get all crates from config
    let crates: Vec<String> = config.crates.keys().cloned().collect();
    println!("📦 Found {} crates to process", crates.len());
    
    // Process crates in parallel
    let results: Vec<_> = crates.par_iter().map(|crate_name| {
        process_single_crate(crate_name, &config)
    }).collect();
    
    // Report results
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (crate_name, result) in crates.iter().zip(results.iter()) {
        match result {
            Ok(_) => {
                println!("✅ {}", crate_name);
                success_count += 1;
            }
            Err(e) => {
                println!("❌ {}: {}", crate_name, e);
                error_count += 1;
            }
        }
    }
    
    println!("\n📊 Summary: {} success, {} errors", success_count, error_count);
    Ok(())
}

fn process_single_crate(crate_name: &str, config: &SplitDeclsConfig) -> Result<()> {
    let crate_path = Path::new("..").join(crate_name);
    let output_dir = Path::new("output2").join(format!("wrapped-{}", crate_name));
    
    // Create CratePaths
    let paths = CratePaths {
        crate_name: crate_name.to_string(),
        crate_path: crate_path.clone(),
        source_files: vec![crate_path.join("src/lib.rs")],
        decls_output_dir: output_dir.clone(),
        build_rs_path: output_dir.join("build.rs"),
        cargo_toml_path: crate_path.join("Cargo.toml"),
        target_config_path: output_dir.join("target_config.toml"),
        output_crate_path: output_dir.clone(),
    };
    
    // Check if source exists
    let lib_rs_path = crate_path.join("src/lib.rs");
    if !lib_rs_path.exists() {
        return Err(anyhow::anyhow!("No src/lib.rs found"));
    }
    
    // Process using library function
    let errors = eager_split_crate(&paths, config)?;
    
    // Log results
    let log_content = if errors.is_empty() {
        format!("✅ Successfully processed {}\n", crate_name)
    } else {
        format!("⚠️ Processed {} with {} warnings\n", crate_name, errors.len())
    };
    
    fs::write(format!("logs/{}.log", crate_name), log_content)?;
    Ok(())
}
