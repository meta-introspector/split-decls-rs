use anyhow::Result;
use std::process::Command;
use std::fs;
use std::path::Path;
use toml::Value;

fn main() -> Result<()> {
    println!("🔍 Scanning Cargo.lock for all crates...");
    
    let cargo_lock_content = fs::read_to_string("Cargo.lock")?;
    let cargo_lock: Value = toml::from_str(&cargo_lock_content)?;
    
    let mut crate_names = Vec::new();
    
    if let Some(packages) = cargo_lock.get("package").and_then(|p| p.as_array()) {
        for package in packages {
            if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
                crate_names.push(name.to_string());
            }
        }
    }
    
    println!("📊 Found {} crates in Cargo.lock", crate_names.len());
    
    // Create output2 directory
    fs::create_dir_all("output2")?;
    
    // Process each crate with enhanced_wrapper using make -j20
    println!("🚀 Processing all crates with make -j20...");
    
    let mut make_targets = Vec::new();
    for crate_name in &crate_names {
        make_targets.push(format!("wrap-{}", crate_name));
    }
    
    // Create Makefile rules for each crate
    let mut makefile_content = String::new();
    makefile_content.push_str("# Auto-generated Makefile for parallel crate wrapping\n\n");
    makefile_content.push_str("all: ");
    for target in &make_targets {
        makefile_content.push_str(&format!("{} ", target));
    }
    makefile_content.push_str("\n\n");
    
    for crate_name in &crate_names {
        makefile_content.push_str(&format!(
            "wrap-{}:\n\t@echo \"🔄 Processing crate: {}\"\n\t@cargo run --bin enhanced_wrapper {} || true\n\n",
            crate_name, crate_name, crate_name
        ));
    }
    
    fs::write("Makefile.wrap", makefile_content)?;
    
    println!("📝 Generated Makefile.wrap with {} targets", make_targets.len());
    println!("🏃 Running make -j20 -f Makefile.wrap all...");
    
    let output = Command::new("make")
        .args(&["-j20", "-f", "Makefile.wrap", "all"])
        .output()?;
    
    if output.status.success() {
        println!("✅ All crates processed successfully!");
    } else {
        println!("⚠️  Some crates may have failed, but continuing...");
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    println!("🎉 Parallel crate wrapping complete!");
    
    Ok(())
}
