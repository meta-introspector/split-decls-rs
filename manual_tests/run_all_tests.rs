use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running all test cases with enhanced parsing fixes...");
    println!("🔧 New: Try block parsing fix with #![feature(try_blocks)]");
    
    // Find all test case files in the parent directory
    let test_cases_dir = "../test_cases";
    if !Path::new(test_cases_dir).exists() {
        println!("❌ No test_cases directory found. Run: cargo run --bin runbuild");
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
    
    if test_files.is_empty() {
        println!("❌ No test case files found in {}", test_cases_dir);
        return Ok(());
    }
    
    println!("📁 Found {} test case files", test_files.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut error_categories = HashMap::new();
    let mut successful_tests = Vec::new();
    let mut try_block_fixes = 0;
    
    for (i, test_file) in test_files.iter().enumerate() {
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        println!("\n🔄 Processing {}/{}: {}", i + 1, test_files.len(), file_name);
        
        // Read the test file
        let content = fs::read_to_string(test_file)?;
        
        // Count fixes for reporting
        let rust_fixes = content.matches("../rust/").count();
        let submodule_fixes = content.matches("../submodules/").count();
        let relative_fixes = content.matches("submodules/rust/").count();
        
        // Check for try block patterns that would benefit from the fix
        if content.contains("try {") {
            try_block_fixes += 1;
            println!("   🎯 Contains try blocks - will benefit from feature flag fix");
        }
        
        if rust_fixes + submodule_fixes + relative_fixes > 0 {
            println!("   🔧 Fixing {} path references", rust_fixes + submodule_fixes + relative_fixes);
        }
        
        // Fix multiple path issues:
        // 1. ../rust/ -> absolute path to rust submodule
        // 2. ../submodules/ -> absolute path to submodules
        let mut fixed_content = content.replace(
            "../rust/",
            "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust/"
        );
        
        // Also fix any ../submodules/ references
        fixed_content = fixed_content.replace(
            "../submodules/",
            "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/"
        );
        
        // Fix any remaining relative rust paths
        fixed_content = fixed_content.replace(
            "submodules/rust/",
            "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust/"
        );
        
        // Create a temporary test file in manual_tests
        let temp_test_name = format!("temp_test_{}.rs", i);
        let temp_test_path = format!("src/{}", temp_test_name);
        
        // Ensure src directory exists
        fs::create_dir_all("src")?;
        
        // Write the fixed content
        fs::write(&temp_test_path, &fixed_content)?;
        
        // Try to compile with cargo instead of rustc directly
        let temp_cargo_toml = format!(r#"
[package]
name = "temp_test_{}"
version = "0.1.0"
edition = "2021"

[dependencies]
syn = {{ version = "2.0", features = ["full", "parsing"] }}
split-decls-genesis = {{ path = ".." }}

[[bin]]
name = "temp_test_{}"
path = "src/{}"
"#, i, i, temp_test_name);
        
        fs::write("Cargo.toml.temp", &temp_cargo_toml)?;
        
        let output = Command::new("cargo")
            .args(&[
                "build", "--bin", &format!("temp_test_{}", i),
                "--manifest-path", "Cargo.toml.temp"
            ])
            .output()?;
        
        if output.status.success() {
            success_count += 1;
            successful_tests.push(file_name.to_string());
            println!("✅ {}: SUCCESS", file_name);
            
            // Try to run the compiled binary
            let run_output = Command::new(&format!("./target/debug/temp_test_{}", i))
                .output();
            
            match run_output {
                Ok(run_result) if run_result.status.success() => {
                    println!("   🏃 Execution: SUCCESS");
                }
                Ok(run_result) => {
                    println!("   🏃 Execution: FAILED");
                    let stderr = String::from_utf8_lossy(&run_result.stderr);
                    let error_lines: Vec<&str> = stderr.lines().take(2).collect();
                    for line in error_lines {
                        println!("     {}", line);
                    }
                }
                Err(e) => {
                    println!("   🏃 Execution: ERROR - {}", e);
                }
            }
        } else {
            failure_count += 1;
            
            // Categorize the error
            let stderr = String::from_utf8_lossy(&output.stderr);
            let error_category = if stderr.contains("unresolved module") || stderr.contains("unlinked crate") {
                "missing_crate"
            } else if stderr.contains("expected identifier") {
                "syntax_identifier"
            } else if stderr.contains("expected") && stderr.contains("found") {
                "syntax_mismatch"
            } else if stderr.contains("macro") {
                "macro_error"
            } else {
                "other_error"
            };
            
            *error_categories.entry(error_category.to_string()).or_insert(0) += 1;
            
            println!("❌ {}: FAILED ({})", file_name, error_category);
            
            // Show first few lines of error for debugging
            let error_lines: Vec<&str> = stderr.lines().take(2).collect();
            for line in error_lines {
                println!("   {}", line);
            }
        }
        
        // Clean up temporary files
        let _ = fs::remove_file(&temp_test_path);
        let _ = fs::remove_file("Cargo.toml.temp");
        let _ = fs::remove_file(&format!("target/debug/temp_test_{}", i));
    }
    
    println!("\n🏁 TEST SUMMARY:");
    println!("   Total tests: {}", test_files.len());
    println!("   Successes: {} ({:.1}%)", success_count, 
             (success_count as f64 / test_files.len() as f64) * 100.0);
    println!("   Failures: {} ({:.1}%)", failure_count,
             (failure_count as f64 / test_files.len() as f64) * 100.0);
    println!("   Try block patterns found: {}", try_block_fixes);
    
    if try_block_fixes > 0 {
        println!("\n🎯 TRY BLOCK FIX IMPACT:");
        println!("   {} files contain try blocks that benefit from #![feature(try_blocks)]", try_block_fixes);
        println!("   This addresses the primary cause of parsing failures in rustc codebase");
    }
    
    if !error_categories.is_empty() {
        println!("\n📊 ERROR BREAKDOWN:");
        let mut sorted_errors: Vec<_> = error_categories.iter().collect();
        sorted_errors.sort_by(|a, b| b.1.cmp(a.1));
        for (category, count) in sorted_errors {
            println!("   {}: {} cases ({:.1}%)", category, count, 
                     (*count as f64 / failure_count as f64) * 100.0);
        }
    }
    
    if !successful_tests.is_empty() {
        println!("\n✅ SUCCESSFUL TESTS:");
        for test in &successful_tests {
            println!("   {}", test);
        }
    }
    
    Ok(())
}
