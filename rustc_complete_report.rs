use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RUSTC_COMPLETE BUILD ANALYSIS REPORT");
    println!("{}", "=".repeat(50));
    
    // Load exclusions
    let exclusions = load_error_exclusions();
    println!("📋 EXCLUSION PATTERNS ({} total):", exclusions.len());
    for (i, exclusion) in exclusions.iter().enumerate() {
        println!("  {}: {}", i + 1, exclusion);
    }
    println!();
    
    // For now, simulate with some test files since we can't load the symbol map easily
    let test_files = vec![
        "../rust/compiler/rustc_ast/src/lib.rs",
        "../rust/compiler/rustc_data_structures/src/lib.rs", 
        "../rust/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs",
        "../rust/compiler/rustc_resolve/src/late_diagnostics.rs",
        "../rust/compiler/rustc_metadata/src/rmeta/mod.rs",
        "../rust/compiler/rustc_hir_analysis/src/lib.rs",
        "../rust/compiler/rustc_span/src/lib.rs",
    ];
    
    println!("📁 TEST FILES: {} files", test_files.len());
    println!();
    
    // Analyze what would happen
    println!("🔄 PROCESSING SIMULATION:");
    let mut processed_count = 0;
    let mut first_exclusion_index = None;
    
    for (i, file) in test_files.iter().enumerate() {
        let would_exclude = should_exclude_file(file, &exclusions);
        let processed_name = format!("processed_{}", 
            file.replace("/", "_").replace(".rs", ".rs"));
        
        if would_exclude {
            if first_exclusion_index.is_none() {
                first_exclusion_index = Some(i);
                println!("  🛑 FIRST EXCLUSION at file #{}: {}", i + 1, file);
                println!("     Processed name: {}", processed_name);
                
                // Find which pattern matched
                for pattern in &exclusions {
                    if exclusions.contains(&processed_name) && pattern == &processed_name {
                        println!("     ✅ Matched exact pattern: {}", pattern);
                        break;
                    } else if processed_name.contains(pattern) {
                        println!("     ✅ Matched prefix pattern: {}", pattern);
                        break;
                    }
                }
                break; // Stop at first exclusion
            }
        } else {
            processed_count += 1;
            println!("  ✅ INCLUDE #{}: {}", i + 1, file);
        }
    }
    
    println!();
    println!("📈 SUMMARY:");
    println!("  Total test files: {}", test_files.len());
    println!("  Would process: {} files ({:.1}%)", 
             processed_count, 
             (processed_count as f64 / test_files.len() as f64) * 100.0);
    
    if let Some(stop_index) = first_exclusion_index {
        println!("  Stop at file: #{} of {} ({:.1}% through list)", 
                 stop_index + 1, 
                 test_files.len(),
                 ((stop_index + 1) as f64 / test_files.len() as f64) * 100.0);
    } else {
        println!("  Would process: ALL files (no exclusions found)");
    }
    
    Ok(())
}

fn load_error_exclusions() -> HashSet<String> {
    let mut exclusions = HashSet::new();
    
    if let Ok(content) = fs::read_to_string("error_list.txt") {
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                exclusions.insert(line.to_string());
            }
        }
    }
    
    exclusions
}

fn should_exclude_file(file_path: &str, exclusions: &HashSet<String>) -> bool {
    let processed_name = format!("processed_{}", 
        file_path.replace("/", "_").replace(".rs", ".rs"));
    
    // Check exact match
    if exclusions.contains(&processed_name) {
        return true;
    }
    
    // Check pattern matches
    for pattern in exclusions {
        if processed_name.contains(pattern) {
            return true;
        }
    }
    
    false
}
