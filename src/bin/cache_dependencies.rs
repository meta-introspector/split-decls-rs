use split_decls_rs::extract_dependencies_from_macro_wrapped_code;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DependencyCache {
    files: HashMap<String, Vec<String>>,
}

fn main() {
    let mut cache = DependencyCache {
        files: HashMap::new(),
    };
    
    let output2_path = Path::new("output2");
    if !output2_path.exists() {
        eprintln!("output2 directory not found");
        return;
    }
    
    process_directory(&output2_path, &mut cache);
    
    let cache_json = serde_json::to_string_pretty(&cache).unwrap();
    fs::write("dependency_cache.json", cache_json).unwrap();
    
    println!("Processed {} files", cache.files.len());
    println!("Cache saved to dependency_cache.json");
}

fn process_directory(dir: &Path, cache: &mut DependencyCache) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_directory(&path, cache);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let deps = extract_dependencies_from_macro_wrapped_code(&content);
                    if !deps.is_empty() {
                        cache.files.insert(
                            path.to_string_lossy().to_string(),
                            deps.into_iter().collect()
                        );
                    }
                }
            }
        }
    }
}
