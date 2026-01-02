// unified_build.rs - Complete replacement for old build.rs
// Combines: runbuild.rs + process_rustc_files.rs + export_symbol_map.rs + run_build.rs transformations
// This is the single expensive operation that generates all code

use split_decls_genesis::{
    symbol_resolver::{find_rust_files_in_dependency_fast, extract_all_symbols_from_file_fast, Symbol},
    dependency_extractor::get_rustc_dependencies,
    build_lib::*
};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Unified Build: Complete rustc processing pipeline");
    
    // Check cache first
    let cache_path = "build_cache.json";
    if Path::new(cache_path).exists() {
        if let Ok(cache_content) = fs::read_to_string(cache_path) {
            if let Ok(cache_data) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                if let Some(timestamp) = cache_data.get("timestamp").and_then(|t| t.as_u64()) {
                    let cache_age = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() - timestamp;
                    
                    if cache_age < 3600 {
                        println!("💾 Found recent cache ({} seconds old), skipping processing", cache_age);
                        return Ok(());
                    }
                }
            }
        }
    }
    
    println!("🔧 Step 1: Processing rustc files with transformations...");
    process_rustc_files()?;
    
    println!("🗺️  Step 2: Exporting symbol map with dependencies...");
    export_symbol_map()?;
    
    println!("💾 Step 3: Creating build cache...");
    create_build_cache()?;
    
    println!("✅ Unified build complete!");
    Ok(())
}

fn process_rustc_files() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_deps = get_rustc_dependencies().unwrap_or_default();
    let mut processed_count = 0;
    let mut total_files = 0;
    
    // Count total files
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        total_files += rust_files.len();
    }
    
    println!("📊 Found {} total files to process", total_files);
    
    // Process each file with transformations
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        println!("📦 Processing {}: {} files", dep.name, rust_files.len());
        
        for file in rust_files {
            if let Ok(content) = fs::read_to_string(&file) {
                // Apply transformations (from run_build.rs)
                let transformed = apply_transformations(&content, &file.to_string_lossy())?;
                
                // Write transformed file to submodules/
                let output_path = get_output_path(&file, &dep.name)?;
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&output_path, transformed)?;
                
                processed_count += 1;
                if processed_count % 100 == 0 {
                    println!("  📊 Processed {}/{} files ({:.1}%)", 
                        processed_count, total_files, 
                        (processed_count as f32 / total_files as f32) * 100.0);
                }
            }
        }
    }
    
    println!("✅ Processed {} files with transformations", processed_count);
    Ok(())
}

fn apply_transformations(content: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = content.to_string();
    
    // Add feature flags for rustc parsing
    if !result.contains("#![feature(try_blocks)]") {
        result = format!("#![feature(try_blocks)]\n{}", result);
    }
    
    // Apply transformations from run_build.rs
    result = add_prelude(&result);
    result = fix_file_paths(&result, file_path);
    result = fix_env_vars(&result);
    result = fix_attribute_spacing(&result);
    result = remove_crate_attrs(&result);
    
    Ok(result)
}

fn export_symbol_map() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_deps = get_rustc_dependencies().unwrap_or_default();
    let mut file_symbols: HashMap<String, Vec<Symbol>> = HashMap::new();
    let mut file_count = 0;
    let mut total_files = 0;
    
    // Count total files
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        total_files += rust_files.len();
    }
    
    // Group symbols by source file
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        
        for file in rust_files {
            file_count += 1;
            if let Ok(symbols) = extract_all_symbols_from_file_fast(&file.to_string_lossy(), &dep.name) {
                for symbol in symbols {
                    file_symbols.entry(symbol.source_file.clone()).or_insert_with(Vec::new).push(symbol);
                }
            }
            
            if file_count % 200 == 0 {
                println!("  📊 Extracted symbols from {}/{} files ({:.1}%)", 
                    file_count, total_files, 
                    (file_count as f32 / total_files as f32) * 100.0);
            }
        }
    }
    
    // Calculate dependencies and sort
    let mut dependency_counts: Vec<(String, usize)> = Vec::new();
    let mut symbol_map: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    
    for (file_path, symbols) in &file_symbols {
        let mut total_deps = 0;
        for symbol in symbols {
            total_deps += symbol.dependencies.len();
            
            let symbol_data = serde_json::json!({
                "name": symbol.name,
                "symbol_type": symbol.symbol_type,
                "source_file": symbol.source_file,
                "crate_name": symbol.crate_name,
                "dependencies": symbol.dependencies
            });
            
            let symbol_key = format!("{}::{}", symbol.crate_name, symbol.name);
            symbol_map.insert(symbol_key, symbol_data);
        }
        dependency_counts.push((file_path.clone(), total_deps));
    }
    
    // Sort by dependency count (0 dependencies first)
    dependency_counts.sort_by_key(|(_, count)| *count);
    
    // Write compressed symbol map
    let output_file = fs::File::create("symbol_map_original.json.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let json_output = serde_json::to_string_pretty(&symbol_map)?;
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    println!("✅ Exported {} symbols to symbol_map_original.json.gz", symbol_map.len());
    println!("📊 Files with 0 dependencies: {}", 
        dependency_counts.iter().filter(|(_, count)| *count == 0).count());
    
    Ok(())
}

fn create_build_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_data = serde_json::json!({
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        "version": "unified_build_v1",
        "status": "complete"
    });
    
    fs::write("build_cache.json", serde_json::to_string_pretty(&cache_data)?)?;
    Ok(())
}

fn get_output_path(input_path: &Path, crate_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Convert input path to output path in submodules/
    let relative_path = input_path.strip_prefix("../rust/")?;
    Ok(PathBuf::from("submodules/rust").join(relative_path))
}

// Transformation functions (from run_build.rs)
fn add_prelude(content: &str) -> String {
    format!("// Generated by unified_build.rs\nuse crate::*;\n\n{}", content)
}

fn fix_file_paths(content: &str, _file_path: &str) -> String {
    content.replace("../", "")
}

fn fix_env_vars(content: &str) -> String {
    content.replace("env!(", "option_env!(")
}

fn fix_attribute_spacing(content: &str) -> String {
    content.replace("# [", "#[")
}

fn remove_crate_attrs(content: &str) -> String {
    content.lines()
        .filter(|line| !line.trim_start().starts_with("#!["))
        .collect::<Vec<_>>()
        .join("\n")
}
