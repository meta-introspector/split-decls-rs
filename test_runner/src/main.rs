use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TestResult {
    name: String,
    success: bool,
    error_type: Option<String>,
    error_message: Option<String>,
    execution_success: bool,
    execution_output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Standalone Test Runner - Running all test cases...");
    
    // Find all test case files
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
    
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut error_categories = HashMap::new();
    let mut successful_tests = Vec::new();
    
    for (i, test_file) in test_files.iter().enumerate() {
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        println!("\n🔄 Processing {}/{}: {}", i + 1, test_files.len(), file_name);
        
        let mut result = TestResult {
            name: file_name.to_string(),
            success: false,
            error_type: extract_error_type(&file_name),
            error_message: None,
            execution_success: false,
            execution_output: None,
        };
        
        // Read and process the test file
        let content = fs::read_to_string(test_file)?;
        
        // Fix path references
        let fixed_content = fix_paths(&content);
        
        // Create temporary test file
        let temp_test_name = format!("test_{}.rs", i);
        let temp_test_path = format!("src/{}", temp_test_name);
        fs::write(&temp_test_path, &fixed_content)?;
        
        // Add binary to Cargo.toml temporarily
        let cargo_toml_addition = format!("\n[[bin]]\nname = \"test_{}\"\npath = \"src/{}\"", i, temp_test_name);
        fs::write("Cargo.toml.backup", fs::read_to_string("Cargo.toml")?)?;
        let mut cargo_content = fs::read_to_string("Cargo.toml")?;
        cargo_content.push_str(&cargo_toml_addition);
        fs::write("Cargo.toml", &cargo_content)?;
        
        // Try to compile
        let output = Command::new("cargo")
            .args(&["build", "--bin", &format!("test_{}", i)])
            .output()?;
        
        // Restore Cargo.toml
        fs::rename("Cargo.toml.backup", "Cargo.toml")?;
        
        if output.status.success() {
            result.success = true;
            success_count += 1;
            successful_tests.push(file_name.to_string());
            println!("✅ {}: COMPILATION SUCCESS", file_name);
            
            // Check if binary actually exists before trying to run it
            let binary_path = format!("target/debug/test_{}", i);
            if Path::new(&binary_path).exists() {
                // Try to run the test
                match run_test(i) {
                    Ok(output) => {
                        result.execution_success = true;
                        result.execution_output = Some(output.clone());
                        println!("   🏃 EXECUTION: SUCCESS");
                        if !output.is_empty() {
                            println!("   📄 Output: {}", output.lines().take(2).collect::<Vec<_>>().join(" | "));
                        }
                    }
                    Err(e) => {
                        result.execution_output = Some(e.clone());
                        println!("   🏃 EXECUTION: FAILED - {}", e.lines().take(1).collect::<Vec<_>>().join(""));
                    }
                }
            } else {
                println!("   ⚠️  Binary not found, skipping execution");
            }
        } else {
            result.error_message = Some(String::from_utf8_lossy(&output.stderr).to_string());
            failure_count += 1;
            
            // Categorize the error
            let stderr = String::from_utf8_lossy(&output.stderr);
            let error_category = categorize_error(&stderr);
            *error_categories.entry(error_category.clone()).or_insert(0) += 1;
            
            println!("❌ {}: COMPILATION FAILED ({})", file_name, error_category);
            println!("   🔍 Error: {}", stderr.lines().take(2).collect::<Vec<_>>().join(" | "));
        }
        
        results.push(result);
        
        // Clean up
        let _ = fs::remove_file(&temp_test_path);
        let _ = fs::remove_file(&format!("target/debug/test_{}", i));
    }
    
    // Print comprehensive summary
    print_summary(test_files.len(), success_count, failure_count, &error_categories, &successful_tests, &results);
    
    Ok(())
}

fn extract_error_type(filename: &str) -> Option<String> {
    if filename.contains("expected_identifier") {
        Some("expected_identifier".to_string())
    } else if filename.contains("expected_comma") {
        Some("expected_comma".to_string())
    } else if filename.contains("expected_expression") {
        Some("expected_expression".to_string())
    } else if filename.contains("unexpected_token") {
        Some("unexpected_token".to_string())
    } else {
        Some("other_parse_error".to_string())
    }
}

fn fix_paths(content: &str) -> String {
    let mut fixed = content.replace(
        "../rust/",
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/"
    );
    fixed = fixed.replace(
        "../submodules/",
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/"
    );
    fixed = fixed.replace(
        "submodules/rust/",
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/"
    );
    fixed
}

fn run_test(index: usize) -> Result<String, String> {
    let output = Command::new(&format!("./target/debug/test_{}", index))
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn categorize_error(error: &str) -> String {
    if error.contains("unresolved module") || error.contains("unlinked crate") {
        "missing_crate".to_string()
    } else if error.contains("expected identifier") {
        "syntax_identifier".to_string()
    } else if error.contains("expected") && error.contains("found") {
        "syntax_mismatch".to_string()
    } else if error.contains("macro") {
        "macro_error".to_string()
    } else if error.contains("type") {
        "type_error".to_string()
    } else if error.contains("borrow") {
        "borrow_error".to_string()
    } else {
        "other_error".to_string()
    }
}

fn print_summary(
    total: usize,
    successes: usize, 
    failures: usize,
    error_categories: &HashMap<String, usize>,
    successful_tests: &[String],
    results: &[TestResult]
) {
    println!("\n{}", "=".repeat(60));
    println!("🏁 COMPREHENSIVE TEST SUMMARY");
    println!("{}", "=".repeat(60));
    
    println!("\n📊 OVERALL RESULTS:");
    println!("   Total tests: {}", total);
    println!("   ✅ Successes: {} ({:.1}%)", successes, 
             (successes as f64 / total as f64) * 100.0);
    println!("   ❌ Failures: {} ({:.1}%)", failures,
             (failures as f64 / total as f64) * 100.0);
    
    if !error_categories.is_empty() {
        println!("\n📊 ERROR BREAKDOWN:");
        let mut sorted_errors: Vec<_> = error_categories.iter().collect();
        sorted_errors.sort_by(|a, b| b.1.cmp(a.1));
        for (category, count) in sorted_errors {
            println!("   {}: {} cases ({:.1}%)", category, count, 
                     (*count as f64 / failures as f64) * 100.0);
        }
    }
    
    if !successful_tests.is_empty() {
        println!("\n✅ SUCCESSFUL TESTS:");
        for test in successful_tests {
            println!("   {}", test);
        }
    }
    
    // Show top failure patterns
    let failed_tests: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed_tests.is_empty() {
        println!("\n❌ FAILED TESTS (showing first 5):");
        for test in failed_tests.iter().take(5) {
            println!("   {} {} - {}", 
                     test.name,
                     test.error_type.as_ref().map(|t| format!("({})", t)).unwrap_or_default(),
                     test.error_message.as_ref()
                         .map(|e| e.lines().next().unwrap_or(""))
                         .unwrap_or("Unknown error"));
        }
        if failed_tests.len() > 5 {
            println!("   ... and {} more failures", failed_tests.len() - 5);
        }
    }
    
    println!("\n{}", "=".repeat(60));
}
