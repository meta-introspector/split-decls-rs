use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;
use std::fs;

#[derive(Debug, Default)]
struct DirectoryStats {
    modules: usize,
    items: usize,
    files: Vec<String>,
}

fn main() -> Result<()> {
    let output2_path = "output2/wrapped-split-decls-rs/src/decls";
    
    println!("📊 Real Output2 Directory Analysis");
    println!("==================================");
    
    let mut dir_stats: HashMap<String, DirectoryStats> = HashMap::new();
    
    // Simple directory walk without walkdir
    fn walk_dir(dir: &Path, base: &Path, stats: &mut HashMap<String, DirectoryStats>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension() == Some("rs".as_ref()) {
                    let relative_path = path.strip_prefix(base)?;
                    let dir_parts: Vec<_> = relative_path.components()
                        .map(|c| c.as_os_str().to_string_lossy())
                        .collect();
                    
                    if dir_parts.len() >= 2 {
                        let dir_key = format!("{}/{}", dir_parts[0], dir_parts[1]);
                        let entry_stats = stats.entry(dir_key).or_default();
                        entry_stats.items += 1;
                        entry_stats.files.push(relative_path.to_string_lossy().to_string());
                    }
                } else if path.is_dir() {
                    walk_dir(&path, base, stats)?;
                }
            }
        }
        Ok(())
    }
    
    let base_path = Path::new(output2_path);
    if base_path.exists() {
        walk_dir(base_path, base_path, &mut dir_stats)?;
    } else {
        println!("❌ Output2 directory not found: {}", output2_path);
        return Ok(());
    }
    
    // Count modules per directory
    for (_, stats) in &mut dir_stats {
        let mut modules = std::collections::HashSet::new();
        for file in &stats.files {
            if let Some(parent) = Path::new(file).parent() {
                modules.insert(parent.to_string_lossy().to_string());
            }
        }
        stats.modules = modules.len();
    }
    
    // Sort by item count
    let mut sorted_dirs: Vec<_> = dir_stats.iter().collect();
    sorted_dirs.sort_by(|a, b| b.1.items.cmp(&a.1.items));
    
    println!("📁 Directory breakdown (top 20):");
    println!("{:<40} {:>8} {:>8}", "Directory", "Modules", "Items");
    println!("{:-<60}", "");
    
    for (dir, stats) in sorted_dirs.iter().take(20) {
        println!("{:<40} {:>8} {:>8}", dir, stats.modules, stats.items);
    }
    
    println!("{:-<60}", "");
    println!("{:<40} {:>8} {:>8}", "TOTAL (all dirs)", 
             dir_stats.values().map(|s| s.modules).sum::<usize>(),
             dir_stats.values().map(|s| s.items).sum::<usize>());
    
    println!("\n📈 Summary:");
    println!("  • {} total directories", dir_stats.len());
    println!("  • {} total declaration files", 
             dir_stats.values().map(|s| s.items).sum::<usize>());
    
    Ok(())
}
