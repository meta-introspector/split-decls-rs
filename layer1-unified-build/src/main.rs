use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use syn::parse_file;

use unified_build::transform_bootstrap::add_bootstrap_features;
use unified_build::transform_jobserver::fix_jobserver_imports;
use unified_build::transform_platform::remove_platform_specific;
use unified_build::transform_tests::remove_test_code;
use unified_build::transform_unused::remove_unused_code;
use unified_build::transform_diagnostics::remove_diagnostic_attributes;
use unified_build::crate_processor::CrateProcessor;
use unified_build::actionable_errors::{ActionableErrorGenerator, print_actionable_error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_path = std::env::args().nth(1)
        .unwrap_or_else(|| "../submodules/rust".to_string());
    
    let output_path = std::env::args().nth(2)
        .unwrap_or_else(|| "./output".to_string());

    println!("🔄 Processing rustc source files from: {}", rustc_path);
    println!("📁 Output directory: {}", output_path);
    
    // Create output directory
    fs::create_dir_all(&output_path)?;
    
    let mut processed_count = 0;
    let mut syn_errors = 0;
    let error_generator = ActionableErrorGenerator::new();

    // Walk through all .rs files in rustc source
    for entry in WalkDir::new(&rustc_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            // Skip test files and examples
            let path_str = e.path().to_string_lossy();
            !path_str.contains("/tests/") && 
            !path_str.contains("/examples/") &&
            !path_str.contains("/benches/") &&
            path_str.contains("/compiler/")
        })
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let relative_path = entry.path().strip_prefix(&rustc_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();

            // Apply transformations
            let mut transformed = content.clone();
            transformed = add_bootstrap_features(&transformed);
            transformed = fix_jobserver_imports(&transformed);
            transformed = remove_platform_specific(&transformed);
            transformed = remove_test_code(&transformed);
            transformed = remove_unused_code(&transformed);
            transformed = remove_diagnostic_attributes(&transformed);

            // IMMEDIATE SYN CHECK after transformation
            if let Err(e) = parse_file(&transformed) {
                let actionable_error = error_generator.generate_actionable_error(&relative_path, &e.to_string());
                print_actionable_error(&actionable_error);
                
                // BISECT TRANSFORMATIONS to find which one broke it
                println!("\n🔍 BISECTING TRANSFORMATIONS...");
                if let Err(bisect_error) = bisect_transformations(&content, &relative_path) {
                    println!("❌ Bisection failed: {}", bisect_error);
                }
                
                syn_errors += 1;
                
                // Stop on first syn error with actionable output
                return Err(format!("Syn parse failed for {}: {}", relative_path, e).into());
            }

            // Write processed file
            let output_file = Path::new(&output_path).join("processed").join(&relative_path);
            
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)?;
            }

            let processed_content = format!(
                "// PROCESSED BY: unified-build\n// SOURCE: {}\n\n{}",
                relative_path,
                transformed
            );

            fs::write(output_file, processed_content)?;
            processed_count += 1;
            
            if processed_count % 100 == 0 {
                println!("📊 Processed {} files (syn errors: {})", processed_count, syn_errors);
            }
        }
    }

    println!("✅ Processed {} files with {} syn errors", processed_count, syn_errors);
    
    // Now process crates in topological order
    println!("\n🔄 Starting crate-by-crate validation...");
    
    let mut processor = CrateProcessor::new();
    processor.discover_crates(&format!("{}/processed", output_path))?;
    processor.process_in_topological_order()?;
    
    // Generate workspace AFTER all crates are created
    processor.generate_workspace(&output_path)?;
    
    println!("\n🎉 All crates processed successfully!");
    
    Ok(())
}

fn bisect_transformations(original_content: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing original file...");
    
    // Test original file first
    if let Err(e) = parse_file(original_content) {
        return Err(format!("❌ Original file already has syn errors: {}", e).into());
    }
    println!("✅ Original file parses correctly");
    
    // Define transformations in order
    let transformations = [
        ("add_bootstrap_features", add_bootstrap_features as fn(&str) -> String),
        ("fix_jobserver_imports", fix_jobserver_imports),
        ("remove_platform_specific", remove_platform_specific),
        ("remove_test_code", remove_test_code),
        ("remove_unused_code", remove_unused_code),
        ("remove_diagnostic_attributes", remove_diagnostic_attributes),
    ];
    
    let mut current_content = original_content.to_string();
    
    // Apply transformations one by one
    for (i, (name, transform_fn)) in transformations.iter().enumerate() {
        println!("\n🔧 [{}/{}] Testing transformation: {}", i + 1, transformations.len(), name);
        
        let transformed = transform_fn(&current_content);
        
        // Test if this transformation breaks parsing
        match parse_file(&transformed) {
            Ok(_) => {
                println!("✅ {} - PASSED", name);
                current_content = transformed;
            }
            Err(e) => {
                println!("❌ {} - FAILED: {}", name, e);
                println!("\n🔍 PROBLEMATIC TRANSFORMATION FOUND!");
                println!("📁 File: {}", file_path);
                println!("🔧 Transformation: {}", name);
                println!("❌ Error: {}", e);
                
                // Show before/after for this transformation
                println!("\n📄 BEFORE transformation:");
                for (i, line) in current_content.lines().take(10).enumerate() {
                    println!("    {:4}: {}", i + 1, line);
                }
                
                println!("\n📄 AFTER transformation:");
                for (i, line) in transformed.lines().take(10).enumerate() {
                    println!("    {:4}: {}", i + 1, line);
                }
                
                return Err(format!("Transformation '{}' broke the file", name).into());
            }
        }
    }
    
    println!("🤔 All transformations passed individually - this shouldn't happen!");
    Ok(())
}
