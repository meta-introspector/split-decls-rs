use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Test Case Compilation Report");
    println!("================================");
    
    let test_cases_dir = "../test_cases";
    if !Path::new(test_cases_dir).exists() {
        println!("❌ No test_cases directory found");
        return Ok(());
    }
    
    let entries = fs::read_dir(test_cases_dir)?;
    let mut test_files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            test_files.push(path);
        }
    }
    
    println!("📁 Found {} test case files\n", test_files.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut error_categories = HashMap::new();
    let mut successful_tests = Vec::new();
    
    for (i, test_file) in test_files.iter().enumerate() {
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        print!("🔄 {}/{}: {} ... ", i + 1, test_files.len(), file_name);
        
        // Read and fix paths
        let content = fs::read_to_string(test_file)?;
        let fixed_content = content
            .replace("../rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/")
            .replace("../submodules/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/")
            .replace("submodules/rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/");
        
        // Create temp test file
        let temp_test_path = format!("src/test_{}.rs", i);
        fs::write(&temp_test_path, &fixed_content)?;
        
        // Add to Cargo.toml temporarily
        let cargo_addition = format!("\n[[bin]]\nname = \"test_{}\"\npath = \"src/test_{}.rs\"", i, i);
        let original_cargo = fs::read_to_string("Cargo.toml")?;
        fs::write("Cargo.toml", format!("{}{}", original_cargo, cargo_addition))?;
        
        // Try to compile
        let output = Command::new("cargo")
            .args(&["build", "--bin", &format!("test_{}", i), "--quiet"])
            .output()?;
        
        // Restore Cargo.toml
        fs::write("Cargo.toml", &original_cargo)?;
        
        if output.status.success() {
            success_count += 1;
            successful_tests.push(file_name.to_string());
            println!("✅ SUCCESS");
        } else {
            failure_count += 1;
            let stderr = String::from_utf8_lossy(&output.stderr);
            let error_category = if stderr.contains("unresolved module") || stderr.contains("unlinked crate") {
                "missing_crate"
            } else if stderr.contains("expected identifier") {
                "syntax_identifier"
            } else if stderr.contains("macro") {
                "macro_error"
            } else {
                "other_error"
            };
            *error_categories.entry(error_category.to_string()).or_insert(0) += 1;
            println!("❌ FAILED ({})", error_category);
        }
        
        // Clean up
        let _ = fs::remove_file(&temp_test_path);
    }
    
    // Print summary
    println!("\n" + &"=".repeat(50));
    println!("📊 COMPILATION SUMMARY");
    println!("{}", "=".repeat(50));
    println!("Total tests: {}", test_files.len());
    println!("✅ Successful: {} ({:.1}%)", success_count, 
             (success_count as f64 / test_files.len() as f64) * 100.0);
    println!("❌ Failed: {} ({:.1}%)", failure_count,
             (failure_count as f64 / test_files.len() as f64) * 100.0);
    
    if !error_categories.is_empty() {
        println!("\n🔍 Error Categories:");
        for (category, count) in error_categories {
            println!("   {}: {} cases", category, count);
        }
    }
    
    if !successful_tests.is_empty() {
        println!("\n✅ Successfully Compiled Tests:");
        for test in successful_tests {
            println!("   {}", test);
        }
    }
    
    Ok(())
}
