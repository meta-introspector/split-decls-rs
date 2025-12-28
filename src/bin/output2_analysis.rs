use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

macro_rules! includedeps {
    () => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/dependency_data.rs"));
    };
}

includedeps!();

#[derive(Debug, Default)]
struct DirectoryStats {
    modules: usize,
    items: usize,
    dependencies: usize,
    files: Vec<String>,
}

fn analyze_output2_structure() -> Result<()> {
    let dependencies = dependency_data!();
    let mut dir_stats: HashMap<String, DirectoryStats> = HashMap::new();
    
    // Get topological order from dependency data
    let build_order = topological_sort(&dependencies);
    
    println!("📊 Output2 Directory Analysis Report");
    println!("====================================");
    
    // Analyze each declaration in dependency order
    for &decl_name in &build_order {
        // Extract directory from output2 structure
        let dir = extract_output2_directory(decl_name);
        let stats = dir_stats.entry(dir).or_default();
        
        stats.items += 1;
        if let Some(deps) = dependencies.get(decl_name) {
            stats.dependencies += deps.len();
        }
        
        // Add file path that would be generated
        let file_path = format!("output2/{}/src/decls/{}.rs", 
                               extract_crate_name(decl_name), decl_name);
        stats.files.push(file_path);
    }
    
    // Count modules per directory
    for (dir, stats) in &mut dir_stats {
        let mut modules = std::collections::HashSet::new();
        for file in &stats.files {
            if let Some(module) = extract_module_from_path(file) {
                modules.insert(module);
            }
        }
        stats.modules = modules.len();
    }
    
    // Sort by complexity
    let mut sorted_dirs: Vec<_> = dir_stats.iter().collect();
    sorted_dirs.sort_by(|a, b| b.1.items.cmp(&a.1.items));
    
    println!("📁 Output2 directory breakdown (in build order):");
    println!("{:<40} {:>8} {:>8} {:>12}", "Directory", "Modules", "Items", "Dependencies");
    println!("{:-<70}", "");
    
    let mut total_modules = 0;
    let mut total_items = 0;
    let mut total_deps = 0;
    
    for (dir, stats) in &sorted_dirs {
        println!("{:<40} {:>8} {:>8} {:>12}", 
                 dir, stats.modules, stats.items, stats.dependencies);
        total_modules += stats.modules;
        total_items += stats.items;
        total_deps += stats.dependencies;
    }
    
    println!("{:-<70}", "");
    println!("{:<40} {:>8} {:>8} {:>12}", "TOTAL", total_modules, total_items, total_deps);
    
    println!("\n📈 Build order summary:");
    println!("  • {} output2 directories", sorted_dirs.len());
    println!("  • {} total modules to generate", total_modules);
    println!("  • {} total declaration files", total_items);
    println!("  • {} dependency relationships", total_deps);
    
    // Show first 10 files in build order
    println!("\n🔧 First 10 files in build order:");
    for (i, &decl_name) in build_order.iter().take(10).enumerate() {
        let crate_name = extract_crate_name(decl_name);
        println!("  {}. output2/{}/src/decls/{}.rs", i + 1, crate_name, decl_name);
    }
    
    Ok(())
}

fn topological_sort(deps: &HashMap<&'static str, Vec<&'static str>>) -> Vec<&'static str> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    
    for (&node, dependencies) in deps {
        in_degree.entry(node).or_insert(0);
        for &dep in dependencies {
            graph.entry(dep).or_default().push(node);
            *in_degree.entry(node).or_insert(0) += 1;
        }
    }
    
    let mut queue: std::collections::VecDeque<&str> = in_degree.iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(node, _)| *node)
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop_front() {
        result.push(node);
        
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    
    result
}

fn extract_output2_directory(decl_name: &str) -> String {
    let crate_name = extract_crate_name(decl_name);
    format!("output2/{}", crate_name)
}

fn extract_crate_name(decl_name: &str) -> String {
    if let Some(pos) = decl_name.find("_decls_") {
        decl_name[..pos].to_string()
    } else {
        "unknown_crate".to_string()
    }
}

fn extract_module_from_path(file_path: &str) -> Option<String> {
    if let Some(pos) = file_path.rfind('/') {
        let filename = &file_path[pos + 1..];
        if let Some(dot_pos) = filename.rfind('.') {
            return Some(filename[..dot_pos].to_string());
        }
    }
    None
}

fn main() -> Result<()> {
    analyze_output2_structure()
}
