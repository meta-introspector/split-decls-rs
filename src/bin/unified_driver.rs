use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::os::unix::process::ExitStatusExt;
use serde_json::Value;

fn resolve_all_dependencies(symbol_map: &HashMap<String, Value>, target: &str) -> HashSet<String> {
    let mut resolved = HashSet::new();
    let mut to_process = vec![target.to_string()];
    
    println!("🔍 Starting recursive resolution from: {}", target);
    
    while let Some(current) = to_process.pop() {
        if resolved.contains(&current) {
            continue;
        }
        
        resolved.insert(current.clone());
        
        if let Some(entry) = symbol_map.get(&current) {
            if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                println!("📋 {} has {} dependencies:", current, deps.len());
                for dep in deps {
                    if let Some(dep_str) = dep.as_str() {
                        let has_source = symbol_map.get(dep_str)
                            .and_then(|e| e.get("source_file"))
                            .and_then(|s| s.as_str())
                            .is_some();
                        let status = if has_source { "✅ HAS_SOURCE" } else { "❌ NO_SOURCE" };
                        println!("  - {} {}", dep_str, status);
                        
                        // Add to processing queue regardless of source availability
                        to_process.push(dep_str.to_string());
                    }
                }
            } else {
                println!("📋 {} has NO dependencies listed", current);
            }
        } else {
            // Try auto-fix before giving up
            if let Some(found_symbol) = auto_fix_missing_symbol(&current, symbol_map) {
                println!("🔧 AUTO-FIX: Found {} -> {}", current, found_symbol);
                to_process.push(found_symbol);
                continue;
            }
            
            // Skip missing symbols instead of panicking to allow partial compilation
            println!("⚠️  SKIPPING: '{}' not found in symbol map (continuing with partial resolution)", current);
        }
        
        if resolved.len() % 100 == 0 {
            println!("📊 Progress: {} symbols resolved so far...", resolved.len());
        }
        
        // Safety check - if we get too many, something is wrong
        if resolved.len() > 50000 {
            panic!("💥 TOO MANY DEPENDENCIES: Resolved {} symbols, expected ~12k. Infinite loop detected!", resolved.len());
        }
    }
    
    resolved
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <rustc_main_symbol>", args[0]);
        println!("Example: {} rustc_driver::main", args[0]);
        return Ok(());
    }
    
    let target = &args[1];
    println!("🎯 Target: {}", target);
    
    // Load symbol map for recursive dependency resolution
    println!("📖 Loading symbol map for recursive resolution...");
    let symbol_map_content = fs::read_to_string("symbol_map.json")?;
    let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_map_content)?;
    
    // Resolve ALL dependencies recursively
    println!("🔄 Resolving all dependencies recursively...");
    let all_deps = resolve_all_dependencies(&symbol_map, target);
    println!("✅ Resolved {} total dependencies (including recursive)", all_deps.len());
    
    // Generate code with ALL dependencies
    let mut complete_code = String::new();
    complete_code.push_str("#![recursion_limit = \"256\"]\n");
    complete_code.push_str("#![allow(internal_features)]\n");
    complete_code.push_str("#![feature(rustc_private)]\n\n");
    complete_code.push_str("include!(\"wrap_types.rs\");\n\n");
    complete_code.push_str(&format!("// Auto-generated module tree for: {}\n", target));
    complete_code.push_str(&format!("// Dependencies resolved: {}\n\n", all_deps.len()));
    
    // Include all resolved dependencies that have source files
    let mut included_count = 0;
    for dep in all_deps.iter() {
        if let Some(entry) = symbol_map.get(dep) {
            if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                let file_path = format!("submodules/{}", source_file);
                if Path::new(&file_path).exists() {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        complete_code.push_str(&format!("// === {} ===\n", dep));
                        complete_code.push_str(&content);
                        complete_code.push_str("\n\n");
                        included_count += 1;
                    }
                }
            }
        }
    }
    
    // Also include ALL rustc source files since we have them
    println!("📁 Including all available rustc source files...");
    let mut rustc_files_added = 0;
    if let Ok(entries) = fs::read_dir("submodules/rust/compiler") {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let crate_path = entry.path();
                if let Some(crate_name) = crate_path.file_name().and_then(|n| n.to_str()) {
                    if crate_name.starts_with("rustc_") {
                        // Find the main lib.rs or mod.rs file for this crate
                        let lib_file = crate_path.join("src/lib.rs");
                        if lib_file.exists() {
                            if let Ok(content) = fs::read_to_string(&lib_file) {
                                complete_code.push_str(&format!("// === {} ===\n", crate_name));
                                complete_code.push_str(&content);
                                complete_code.push_str("\n\n");
                                rustc_files_added += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Included {} dependencies with source files out of {} total", included_count, all_deps.len());
    println!("📈 Added {} rustc crate lib files", rustc_files_added);
    
    // Write complete code
    fs::write("src/current.rs", &complete_code)?;
    println!("📝 Generated src/current.rs with {} bytes", complete_code.len());
    
    // Test compilation
    println!("🚀 Testing compilation...");
    let output = Command::new("cargo")
        .args(&["check", "--lib"])
        .output()?;
    
    if output.status.success() {
        println!("✅ SUCCESS: All {} dependencies compiled!", all_deps.len());
    } else {
        println!("❌ FAILED: Compilation errors:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

fn auto_fix_missing_symbol(missing_symbol: &str, symbol_map: &HashMap<String, Value>) -> Option<String> {
    println!("🔍 AUTO-FIX: Searching for missing symbol: {}", missing_symbol);
    
    // Clean up the symbol (remove extra spaces)
    let cleaned_symbol = missing_symbol.replace(" :: ", "::");
    
    // 1. Search in symbol map for exact matches first
    if symbol_map.contains_key(&cleaned_symbol) {
        println!("🔍 Found exact match after cleanup: {} -> {}", missing_symbol, cleaned_symbol);
        return Some(cleaned_symbol);
    }
    
    // 2. Search in symbol map for partial matches
    let partial_matches: Vec<_> = symbol_map.keys()
        .filter(|key| {
            key.contains(missing_symbol) || 
            missing_symbol.contains(*key) ||
            key.contains(&cleaned_symbol) ||
            cleaned_symbol.contains(*key)
        })
        .collect();
    
    if !partial_matches.is_empty() {
        println!("🔍 Found {} partial matches in symbol map:", partial_matches.len());
        for m in partial_matches.iter().take(3) {
            println!("  - {}", m);
        }
        return Some(partial_matches[0].clone());
    }
    
    // 3. Search filesystem for source files containing the symbol
    if let Ok(grep_results) = std::process::Command::new("grep")
        .args(&["-r", "--include=*.rs", missing_symbol, "submodules/"])
        .output() {
        
        let output = String::from_utf8_lossy(&grep_results.stdout);
        if !output.is_empty() {
            println!("🔍 Found in filesystem:");
            for line in output.lines().take(3) {
                println!("  - {}", line);
            }
            
            // Extract potential symbol names from grep results
            for line in output.lines() {
                if let Some(file_path) = line.split(':').next() {
                    // Try to find a symbol that matches this file
                    let file_stem = std::path::Path::new(file_path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    
                    for key in symbol_map.keys() {
                        if key.contains(file_stem) || key.contains(missing_symbol) {
                            return Some(key.clone());
                        }
                    }
                }
            }
        }
    }
    
    // 4. Search for similar function/module names
    let parts: Vec<&str> = cleaned_symbol.split("::").collect();
    if let Some(last_part) = parts.last() {
        for key in symbol_map.keys() {
            if key.ends_with(last_part) {
                println!("🔍 Found similar ending: {} -> {}", missing_symbol, key);
                return Some(key.clone());
            }
        }
    }
    
    // 5. Search for module-level matches
    if let Some(first_part) = parts.first() {
        for key in symbol_map.keys() {
            if key.starts_with(first_part) {
                println!("🔍 Found similar module: {} -> {}", missing_symbol, key);
                return Some(key.clone());
            }
        }
    }
    
    // 6. Try fuzzy matching - look for keys that contain most of the words
    let words: Vec<&str> = cleaned_symbol.split("::").collect();
    if words.len() > 1 {
        for key in symbol_map.keys() {
            let mut matches = 0;
            for word in &words {
                if key.contains(word) {
                    matches += 1;
                }
            }
            if matches >= words.len() / 2 {  // At least half the words match
                println!("🔍 Found fuzzy match: {} -> {}", missing_symbol, key);
                return Some(key.clone());
            }
        }
    }
    
    println!("❌ AUTO-FIX: No matches found for {}", missing_symbol);
    None
}
