use anyhow::Result;
use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn extract_dependencies_from_file(file_path: &str) -> Result<HashSet<String>> {
    let content = fs::read_to_string(file_path)?;
    let mut deps = HashSet::new();
    
    // Extract use statements
    let use_regex = Regex::new(r"use\s+([^;]+);")?;
    for cap in use_regex.captures_iter(&content) {
        if let Some(use_stmt) = cap.get(1) {
            let use_text = use_stmt.as_str();
            // Extract crate names
            if let Some(crate_name) = use_text.split("::").next() {
                if !crate_name.starts_with("std") && !crate_name.starts_with("core") {
                    deps.insert(crate_name.to_string());
                }
            }
        }
    }
    
    // Extract macro calls
    let macro_regex = Regex::new(r"(\w+)!")?;
    for cap in macro_regex.captures_iter(&content) {
        if let Some(macro_name) = cap.get(1) {
            deps.insert(format!("macro_{}", macro_name.as_str()));
        }
    }
    
    // Extract type references
    let type_regex = Regex::new(r"\b([A-Z][a-zA-Z0-9_]*)\b")?;
    for cap in type_regex.captures_iter(&content) {
        if let Some(type_name) = cap.get(1) {
            deps.insert(format!("type_{}", type_name.as_str()));
        }
    }
    
    Ok(deps)
}

fn analyze_partition(partition_file: &str) -> Result<()> {
    let content = fs::read_to_string(partition_file)?;
    let mut all_deps = HashSet::new();
    
    // Extract file paths from partition
    for line in content.lines() {
        if line.starts_with("// - ") {
            let file_path = line.strip_prefix("// - ").unwrap();
            let full_path = format!("/home/mdupont/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/{}", file_path);
            
            if let Ok(deps) = extract_dependencies_from_file(&full_path) {
                all_deps.extend(deps);
            }
        }
    }
    
    // Generate dependency list
    let partition_name = partition_file.strip_suffix(".rs").unwrap_or(partition_file);
    let deps_file = format!("{}_deps.txt", partition_name);
    
    let mut sorted_deps: Vec<_> = all_deps.into_iter().collect();
    sorted_deps.sort();
    
    fs::write(&deps_file, sorted_deps.join("\n"))?;
    println!("Generated {} with {} dependencies", deps_file, sorted_deps.len());
    
    Ok(())
}

fn main() -> Result<()> {
    analyze_partition("partition_1.rs")?;
    Ok(())
}
