// Test build.rs exclusion system
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing exclusion system...");
    
    // Test load_error_exclusions
    let exclusions = load_error_exclusions();
    println!("📊 Loaded {} exclusions", exclusions.len());
    for exclusion in &exclusions {
        println!("  - {}", exclusion);
    }
    
    // Test should_exclude_file
    let test_files = vec![
        "../rust/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs",
        "../rust/compiler/rustc_resolve/src/late_diagnostics.rs",
        "../rust/compiler/rustc_metadata/src/rmeta/mod.rs",
        "../rust/compiler/rustc_ast/src/lib.rs",
    ];
    
    for file in test_files {
        let excluded = should_exclude_file(file, &exclusions);
        println!("🔍 {} -> {}", file, if excluded { "EXCLUDED" } else { "INCLUDED" });
    }
    
    Ok(())
}

fn load_error_exclusions() -> std::collections::HashSet<String> {
    let mut exclusions = std::collections::HashSet::new();
    
    if let Ok(content) = fs::read_to_string("error_list.txt") {
        println!("📋 Loading exclusions from error_list.txt:");
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                println!("  - {}", line);
                exclusions.insert(line.to_string());
            }
        }
    } else {
        println!("⚠️  No error_list.txt found");
    }
    
    exclusions
}

fn should_exclude_file(file_path: &str, exclusions: &std::collections::HashSet<String>) -> bool {
    let processed_name = format!("processed_{}", 
        file_path.replace("/", "_").replace(".rs", ".rs"));
    
    println!("🔍 Checking exclusion for: {} -> {}", file_path, processed_name);
    
    // Check exact match
    if exclusions.contains(&processed_name) {
        println!("✅ EXCLUDED (exact match): {}", processed_name);
        return true;
    }
    
    // Check pattern matches
    for pattern in exclusions {
        if processed_name.contains(pattern) {
            println!("✅ EXCLUDED (pattern match '{}'): {}", pattern, processed_name);
            return true;
        }
    }
    
    println!("❌ NOT EXCLUDED: {}", processed_name);
    false
}
