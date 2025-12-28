use std::collections::HashMap;
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
}

fn extract_directory(decl_name: &str) -> String {
    // Extract directory from declaration names like "crate_name_decls_item_name"
    if let Some(pos) = decl_name.find("_decls_") {
        let crate_part = &decl_name[..pos];
        // Convert underscores to path separators for readability
        crate_part.replace('_', "/")
    } else {
        "root".to_string()
    }
}

fn main() -> Result<()> {
    let dependencies = dependency_data!();
    let mut dir_stats: HashMap<String, DirectoryStats> = HashMap::new();
    
    println!("📊 Directory Analysis Report");
    println!("============================");
    
    // Analyze all declarations
    for (&decl, deps) in &dependencies {
        let dir = extract_directory(decl);
        let stats = dir_stats.entry(dir).or_default();
        
        stats.items += 1;
        stats.dependencies += deps.len();
    }
    
    // Count unique modules per directory
    let mut dir_modules: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
    for &decl in dependencies.keys() {
        let dir = extract_directory(decl);
        let module = if let Some(pos) = decl.rfind('_') {
            decl[..pos].to_string()
        } else {
            decl.to_string()
        };
        dir_modules.entry(dir).or_default().insert(module);
    }
    
    for (dir, modules) in &dir_modules {
        if let Some(stats) = dir_stats.get_mut(dir) {
            stats.modules = modules.len();
        }
    }
    
    // Sort directories by item count
    let mut sorted_dirs: Vec<_> = dir_stats.iter().collect();
    sorted_dirs.sort_by(|a, b| b.1.items.cmp(&a.1.items));
    
    println!("📁 Directory breakdown:");
    println!("{:<30} {:>8} {:>8} {:>12}", "Directory", "Modules", "Items", "Dependencies");
    println!("{:-<60}", "");
    
    let mut total_modules = 0;
    let mut total_items = 0;
    let mut total_deps = 0;
    
    for (dir, stats) in &sorted_dirs {
        println!("{:<30} {:>8} {:>8} {:>12}", 
                 dir, stats.modules, stats.items, stats.dependencies);
        total_modules += stats.modules;
        total_items += stats.items;
        total_deps += stats.dependencies;
    }
    
    println!("{:-<60}", "");
    println!("{:<30} {:>8} {:>8} {:>12}", "TOTAL", total_modules, total_items, total_deps);
    
    println!("\n📈 Summary:");
    println!("  • {} directories analyzed", sorted_dirs.len());
    println!("  • {} total modules", total_modules);
    println!("  • {} total items", total_items);
    println!("  • {} total dependencies", total_deps);
    println!("  • {:.2} avg dependencies per item", total_deps as f32 / total_items as f32);
    
    // Top 5 directories by complexity
    println!("\n🔥 Top 5 most complex directories:");
    for (dir, stats) in sorted_dirs.iter().take(5) {
        let complexity = stats.dependencies as f32 / stats.items as f32;
        println!("  {} - {:.2} deps/item ({} items)", dir, complexity, stats.items);
    }
    
    Ok(())
}
