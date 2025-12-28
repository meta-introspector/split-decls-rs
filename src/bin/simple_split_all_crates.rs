use anyhow::Result;
use std::process::Command;
use std::path::Path;
use toml::Value;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Reading split-decls-rs.toml configuration...");
    
    let config_content = fs::read_to_string("split-decls-rs.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    let crate_paths = config
        .get("crate_path_overrides")
        .and_then(|v| v.as_table())
        .ok_or_else(|| anyhow::anyhow!("No crate_path_overrides found in config"))?;
    
    println!("📦 Found {} crates to process", crate_paths.len());
    
    let mut success_count = 0;
    let mut fail_count = 0;
    
    for (crate_name, path_value) in crate_paths {
        let crate_path = path_value.as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path for crate {}", crate_name))?;
        
        // Skip if path doesn't exist
        if !Path::new(crate_path).exists() {
            println!("⚠️  Skipping {} - path doesn't exist: {}", crate_name, crate_path);
            continue;
        }
        
        let lib_rs_path = Path::new(crate_path).join("src/lib.rs");
        let main_rs_path = Path::new(crate_path).join("src/main.rs");
        
        // Check for any .rs files in src/
        let src_dir = Path::new(crate_path).join("src");
        let has_rust_files = if src_dir.exists() {
            std::fs::read_dir(&src_dir)
                .map(|entries| {
                    entries.filter_map(|e| e.ok())
                        .any(|entry| {
                            entry.path().extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| ext == "rs")
                                .unwrap_or(false)
                        })
                })
                .unwrap_or(false)
        } else {
            false
        };
        
        // Skip if no Rust files found
        if !has_rust_files {
            println!("⚠️  Skipping {} - no .rs files found in: {}", crate_name, src_dir.display());
            continue;
        }
        
        // Determine which file to use as entry point
        let entry_file = if lib_rs_path.exists() {
            "lib.rs"
        } else if main_rs_path.exists() {
            "main.rs"
        } else {
            println!("⚠️  Skipping {} - no lib.rs or main.rs found, but has other .rs files", crate_name);
            continue;
        };
        
        println!("🔄 Processing: {} ({}) - using {}", crate_name, crate_path, entry_file);
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "simple_split", "--", crate_path, "--output-dir", "output2"])
            .output()?;
        
        if output.status.success() {
            success_count += 1;
            println!("✅ {} - SUCCESS", crate_name);
        } else {
            fail_count += 1;
            println!("❌ {} - FAILED", crate_name);
            if !output.stderr.is_empty() {
                println!("   Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("   ✅ Success: {}", success_count);
    println!("   ❌ Failed: {}", fail_count);
    println!("   📁 Output: output2/wrapped-*/src/decls/");
    
    Ok(())
}
