use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processed_dir = std::env::args().nth(1)
        .unwrap_or_else(|| "./output/processed".to_string());
    
    println!("🔍 DISCOVERING RUSTC CRATES IN TOPOLOGICAL ORDER");
    println!("📁 Processing directory: {}", processed_dir);
    println!("{}", "═".repeat(60));
    
    let mut crates = discover_crates(&processed_dir)?;
    let dependency_levels = calculate_dependency_levels(&crates);
    
    // Sort by dependency level
    let mut sorted_crates: Vec<_> = crates.keys().collect();
    sorted_crates.sort_by_key(|name| dependency_levels.get(*name).unwrap_or(&0));
    
    println!("\n📊 CRATES IN TOPOLOGICAL ORDER:");
    println!("{:<4} {:<30} {:<10} {}", "Lvl", "Crate Name", "Files", "Dependencies");
    println!("{}", "─".repeat(80));
    
    for crate_name in sorted_crates {
        let level = dependency_levels.get(crate_name).unwrap_or(&0);
        let file_count = crates.get(crate_name).unwrap().len();
        let deps = get_basic_dependencies(crate_name);
        
        println!("{:<4} {:<30} {:<10} {}", 
            level, 
            crate_name, 
            file_count,
            if deps.is_empty() { "none".to_string() } else { deps.join(", ") }
        );
    }
    
    println!("\n📈 SUMMARY:");
    println!("Total crates: {}", crates.len());
    println!("Max dependency level: {}", dependency_levels.values().max().unwrap_or(&0));
    
    Ok(())
}

fn discover_crates(processed_dir: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut crates = HashMap::new();
    let compiler_dir = Path::new(processed_dir).join("compiler");
    
    if !compiler_dir.exists() {
        return Err(format!("Compiler directory not found: {}", compiler_dir.display()).into());
    }
    
    for entry in fs::read_dir(&compiler_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_name = entry.file_name().to_string_lossy().to_string();
            
            // Find all .rs files in this crate
            let mut files = Vec::new();
            collect_rs_files(&entry.path(), &mut files)?;
            
            crates.insert(crate_name, files);
        }
    }
    
    Ok(crates)
}

fn collect_rs_files(dir: &Path, files: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.file_name().unwrap().to_string_lossy().to_string());
        }
    }
    Ok(())
}

fn calculate_dependency_levels(crates: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut levels = HashMap::new();
    
    // Basic heuristic: assign levels based on common rustc dependency patterns
    for crate_name in crates.keys() {
        let level = match crate_name.as_str() {
            // Level 0: Foundation crates (no dependencies)
            name if name.contains("span") || name.contains("index") || name.contains("data_structures") => 0,
            name if name.contains("abi") || name.contains("target") => 0,
            
            // Level 1: Basic AST and parsing
            name if name.contains("ast") && !name.contains("passes") && !name.contains("lowering") => 1,
            name if name.contains("feature") || name.contains("errors") => 1,
            name if name.contains("parse") || name.contains("lexer") => 1,
            
            // Level 2: AST processing and expansion
            name if name.contains("expand") || name.contains("builtin_macros") => 2,
            name if name.contains("ast_passes") || name.contains("ast_lowering") => 2,
            name if name.contains("session") => 2,
            
            // Level 3: HIR and middle-level IR
            name if name.contains("hir") && !name.contains("analysis") && !name.contains("typeck") => 3,
            name if name.contains("middle") => 3,
            name if name.contains("query") => 3,
            
            // Level 4: Type checking and analysis
            name if name.contains("hir_analysis") || name.contains("hir_typeck") => 4,
            name if name.contains("infer") || name.contains("trait_selection") => 4,
            name if name.contains("resolve") => 4,
            
            // Level 5: Advanced analysis
            name if name.contains("borrowck") || name.contains("const_eval") => 5,
            name if name.contains("privacy") || name.contains("lint") => 5,
            name if name.contains("ty_utils") => 5,
            
            // Level 6: MIR and optimization
            name if name.contains("mir") => 6,
            name if name.contains("monomorphize") => 6,
            name if name.contains("passes") && !name.contains("ast") => 6,
            
            // Level 7: Code generation
            name if name.contains("codegen") => 7,
            name if name.contains("metadata") => 7,
            
            // Level 8: High-level interfaces
            name if name.contains("interface") => 8,
            name if name.contains("driver") => 9,
            
            // Default: mid-level
            _ => 3,
        };
        
        levels.insert(crate_name.clone(), level);
    }
    
    levels
}

fn get_basic_dependencies(crate_name: &str) -> Vec<String> {
    // Basic dependency inference based on rustc architecture
    match crate_name {
        name if name.contains("driver") => vec!["interface".to_string(), "session".to_string()],
        name if name.contains("interface") => vec!["middle".to_string(), "codegen_ssa".to_string()],
        name if name.contains("hir_analysis") => vec!["middle".to_string(), "hir".to_string()],
        name if name.contains("middle") => vec!["span".to_string(), "data_structures".to_string()],
        _ => vec![],
    }
}
