use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use syn::{File, Item};
use quote::ToTokens;

thread_local! {
    static VISITED_FILES: RefCell<HashSet<PathBuf>> = RefCell::new(HashSet::new());
}

#[derive(Debug, Clone)]
struct WrappedItem {
    name: String,
    module_path: String,
    item_type: String,
    macro_name: String,
    dependencies: HashSet<String>,
    provides: HashSet<String>,
    tokens: String,
}

#[derive(Debug, Clone)]
struct ModuleMacros {
    module_name: String,
    items: Vec<String>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    // Parse arguments
    let mut crate_path: Option<&str> = None;
    let mut output_dir = "output2";
    let mut config_file: Option<&str> = None;
    let mut recurse = false;
    let mut jobs = 1;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output-dir" => {
                if i + 1 < args.len() {
                    output_dir = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("❌ --output-dir requires a value");
                    std::process::exit(1);
                }
            }
            "--config" => {
                if i + 1 < args.len() {
                    config_file = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("❌ --config requires a value");
                    std::process::exit(1);
                }
            }
            "--recurse" => {
                recurse = true;
                i += 1;
            }
            "--jobs" => {
                if i + 1 < args.len() {
                    jobs = args[i + 1].parse().unwrap_or(1);
                    i += 2;
                } else {
                    eprintln!("❌ --jobs requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                if crate_path.is_none() {
                    crate_path = Some(&args[i]);
                }
                i += 1;
            }
        }
    }
    
    if recurse && config_file.is_some() {
        // Process all crates from config file
        process_all_crates_from_config(config_file.unwrap(), output_dir, jobs)
    } else if let Some(path) = crate_path {
        // Process single crate
        process_single_crate(Path::new(path), Path::new(output_dir))
    } else {
        eprintln!("Usage: {} <crate_path> [--output-dir <dir>] [--config <file> --recurse --jobs <n>]", args[0]);
        std::process::exit(1);
    }
}

fn process_all_crates_from_config(config_file: &str, output_dir: &str, jobs: usize) -> Result<()> {
    println!("🚀 Simple Split: Processing all crates from config");
    println!("📋 Config: {}", config_file);
    println!("📁 Output: {}", output_dir);
    println!("🔧 Jobs: {}", jobs);
    
    // Set rayon thread pool
    rayon::ThreadPoolBuilder::new().num_threads(jobs).build_global().unwrap();
    
    // Load config
    let config_content = fs::read_to_string(config_file)?;
    let config: toml::Value = toml::from_str(&config_content)?;
    
    // Extract crate names
    let crates = if let Some(crates_array) = config.get("crates").and_then(|v| v.as_array()) {
        crates_array.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<_>>()
    } else if let Some(wrapping) = config.get("wrapping") {
        if let Some(crates_array) = wrapping.get("crates").and_then(|v| v.as_array()) {
            crates_array.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<_>>()
        } else if let Some(crates_table) = wrapping.get("crates").and_then(|v| v.as_table()) {
            crates_table.keys().cloned().collect::<Vec<_>>()
        } else {
            return Err(anyhow::anyhow!("No crates found in config"));
        }
    } else {
        return Err(anyhow::anyhow!("No crates found in config"));
    };
    
    println!("📦 Found {} crates to process", crates.len());
    
    // Create logs directory
    fs::create_dir_all("logs")?;
    
    // Process crates in parallel
    let results: Vec<_> = crates.par_iter().map(|crate_name| {
        // Find all Cargo.toml files for this crate
        let search_paths = vec![
            Path::new("..").join(crate_name).to_path_buf(),
        ];
        
        let mut found_crates = Vec::new();
        
        // Search for the crate in common locations
        for search_path in search_paths {
            if let Ok(entries) = std::fs::read_dir("..") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Look for crate_name anywhere in the directory tree
                        if let Ok(output) = std::process::Command::new("find")
                            .arg(&path)
                            .arg("-name")
                            .arg("Cargo.toml")
                            .arg("-path")
                            .arg(&format!("*{}*", crate_name))
                            .output() {
                            let cargo_tomls = String::from_utf8_lossy(&output.stdout);
                            for cargo_toml in cargo_tomls.lines() {
                                if !cargo_toml.is_empty() {
                                    let crate_dir = Path::new(cargo_toml).parent().unwrap();
                                    found_crates.push(crate_dir.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If no specific matches, try direct path
        if found_crates.is_empty() {
            let direct_path = Path::new("..").join(crate_name);
            if direct_path.join("Cargo.toml").exists() {
                found_crates.push(direct_path);
            }
        }
        
        // Process all found crates
        let mut all_success = true;
        let crate_count = found_crates.len();
        for crate_path in found_crates {
            let output_path = Path::new(output_dir).join(format!("wrapped-{}", crate_path.file_name().unwrap().to_str().unwrap()));
            
            match process_single_crate(&crate_path, &output_path) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ {}: {}", crate_path.display(), e);
                    all_success = false;
                }
            }
        }
        
        if crate_count == 0 {
            eprintln!("❌ {}: No Cargo.toml found", crate_name);
            all_success = false;
        }
        
        let log_content = if all_success && crate_count > 0 {
            format!("✅ Successfully processed {} (found {} subcrates)\n", crate_name, crate_count)
        } else {
            format!("❌ Error processing {}\n", crate_name)
        };
        let _ = fs::write(format!("logs/{}.log", crate_name), log_content);
        
        (crate_name.clone(), all_success && crate_count > 0)
    }).collect();
    
    // Report results
    let success_count = results.iter().filter(|(_, success)| *success).count();
    let error_count = results.len() - success_count;
    
    println!("\n📊 Summary: {} success, {} errors", success_count, error_count);
    
    for (crate_name, success) in results {
        if success {
            println!("✅ {}", crate_name);
        } else {
            println!("❌ {}", crate_name);
        }
    }
    
    Ok(())
}

fn process_single_crate(crate_path: &Path, output_dir: &Path) -> Result<()> {
    println!("🚀 Simple Split: Wrapping all items in macros");
    println!("📂 Crate: {}", crate_path.display());
    println!("📁 Output: {}", output_dir.display());

    let mut wrapped_items = HashMap::new();
    let mut modules = HashMap::new();
    let mut count = 0;

    // Look for Cargo.toml first
    let cargo_toml = crate_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow::anyhow!("No Cargo.toml found in {}", crate_path.display()));
    }

    // Try to find the main source file (lib.rs or main.rs)
    let lib_rs = crate_path.join("src/lib.rs");
    let main_rs = crate_path.join("src/main.rs");
    
    let source_file = if lib_rs.exists() {
        lib_rs
    } else if main_rs.exists() {
        main_rs
    } else {
        return Err(anyhow::anyhow!("No src/lib.rs or src/main.rs found in {}", crate_path.display()));
    };

    // Process the crate starting from the main source file
    let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
    process_file_recursively(&source_file, crate_path, output_dir, &mut wrapped_items, &mut modules, "crate", &mut count)?;

    // Generate macro files
    generate_macro_files(output_dir, crate_name, &wrapped_items, &modules)?;

    println!("✅ Wrapped {} items in {} modules", wrapped_items.len(), modules.len());
    Ok(())
}

pub fn process_file_recursively(
    file_path: &Path,
    crate_root: &Path,
    output_dir: &Path,
    wrapped_items: &mut HashMap<String, WrappedItem>,
    modules: &mut HashMap<String, ModuleMacros>,
    current_module: &str,
    count: &mut usize,
) -> Result<()> {
    // Prevent infinite recursion by tracking visited files
    let already_visited = VISITED_FILES.with(|visited| {
        let mut visited = visited.borrow_mut();
        if visited.contains(file_path) {
            true
        } else {
            visited.insert(file_path.to_path_buf());
            false
        }
    });
    
    if already_visited {
        return Ok(());
    }
    
    println!("📖 Processing file: {} (module: {})", file_path.display(), current_module);
    
    let content = fs::read_to_string(file_path)?;
    let parsed: File = syn::parse_file(&content)?;
    
    let mut module_macros = ModuleMacros {
        module_name: current_module.to_string(),
        items: Vec::new(),
    };
    
    // Process ALL items and wrap them in macros
    for item in &parsed.items {
        *count += 1;
        
        let (base_name, item_type) = match &item {
            Item::Use(use_item) => {
                let use_name = format!("use_{}", *count);
                let is_pub = matches!(use_item.vis, syn::Visibility::Public(_));
                println!("  📥 Found {} use: {}", if is_pub { "pub" } else { "private" }, use_name);
                (use_name, if is_pub { "pub_use" } else { "use" })
            },
            Item::Fn(f) => {
                println!("  📝 Found function: {}", f.sig.ident);
                (f.sig.ident.to_string(), "function")
            },
            Item::Struct(s) => {
                println!("  🏗️  Found struct: {}", s.ident);
                (s.ident.to_string(), "struct")
            },
            Item::Enum(e) => {
                println!("  🔢 Found enum: {}", e.ident);
                (e.ident.to_string(), "enum")
            },
            Item::Trait(t) => {
                println!("  🎭 Found trait: {}", t.ident);
                (t.ident.to_string(), "trait")
            },
            Item::Impl(_) => {
                let impl_name = format!("impl_{}", *count);
                println!("  🔧 Found impl: {}", impl_name);
                (impl_name, "impl")
            },
            Item::Type(t) => {
                println!("  📋 Found type alias: {}", t.ident);
                (t.ident.to_string(), "type")
            },
            Item::Const(c) => {
                println!("  🔒 Found const: {}", c.ident);
                (c.ident.to_string(), "const")
            },
            Item::Static(s) => {
                println!("  📌 Found static: {}", s.ident);
                (s.ident.to_string(), "static")
            },
            Item::Mod(m) => {
                println!("  📦 Found module: {}", m.ident);
                let mod_name = format!("{}::{}", current_module, m.ident);
                
                // Process module recursively if external
                if m.content.is_none() {
                    if let Ok(Some(mod_path)) = find_module_file(file_path, &m.ident.to_string()) {
                        println!("    → External module, processing: {}", mod_path.display());
                        process_file_recursively(&mod_path, crate_root, output_dir, wrapped_items, modules, &mod_name, count)?;
                    }
                }
                (m.ident.to_string(), "module")
            },
            Item::Macro(m) => {
                let macro_name = if let Some(ident) = &m.ident {
                    println!("  🪄 Found macro: {}", ident);
                    ident.to_string()
                } else {
                    let name = format!("macro_{}", *count);
                    println!("  🪄 Found unnamed macro: {}", name);
                    name
                };
                (macro_name, "macro")
            },
            _ => {
                let other_name = format!("other_{}", *count);
                println!("  ❓ Found other item: {}", other_name);
                (other_name, "other")
            },
        };
        
        // Create unique macro name: Dep<ModuleName><ItemName>
        let safe_module = current_module.replace("::", "_").replace("-", "_");
        let safe_name = base_name.replace("-", "_").replace(":", "_");
        let macro_name = format!("Dep{}{}", safe_module, safe_name);
        
        // Extract dependencies and provides (simplified for now)
        let mut dependencies = HashSet::new();
        let mut provides = HashSet::new();
        
        // For now, assume the item provides its own name
        provides.insert(base_name.clone());
        
        let wrapped_item = WrappedItem {
            name: base_name.clone(),
            module_path: current_module.to_string(),
            item_type: item_type.to_string(),
            macro_name: macro_name.clone(),
            dependencies,
            provides,
            tokens: item.to_token_stream().to_string(),
        };
        
        wrapped_items.insert(macro_name.clone(), wrapped_item);
        module_macros.items.push(macro_name);
    }
    
    modules.insert(current_module.to_string(), module_macros);
    Ok(())
}

fn find_module_file(current_file: &Path, module_name: &str) -> Result<Option<PathBuf>> {
    let parent = current_file.parent().unwrap();
    
    // Try module_name.rs
    let mod_file = parent.join(format!("{}.rs", module_name));
    if mod_file.exists() {
        return Ok(Some(mod_file));
    }
    
    // Try module_name/mod.rs
    let mod_dir = parent.join(module_name).join("mod.rs");
    if mod_dir.exists() {
        return Ok(Some(mod_dir));
    }
    
    Ok(None)
}

pub fn generate_macro_files(
    output_dir: &Path,
    crate_name: &str,
    wrapped_items: &HashMap<String, WrappedItem>,
    modules: &HashMap<String, ModuleMacros>,
) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    
    let wrapped_crate_name = format!("wrapped-{}", crate_name);
    let crate_dir = output_dir.join(&wrapped_crate_name);
    fs::create_dir_all(&crate_dir.join("src"))?;
    
    // Generate individual macro files for each item
    for (macro_name, item) in wrapped_items {
        let macro_content = format!(
            "// Generated macro for {} ({})\n\
             macro_rules! {} {{\n\
             () => {{\n\
             // Module: {}\n\
             // Provides: {:?}\n\
             // Dependencies: {:?}\n\
             {}\n\
             }};\n\
             }}\n",
            item.name,
            item.item_type,
            macro_name,
            item.module_path,
            item.provides,
            item.dependencies,
            item.tokens
        );
        
        let macro_file = crate_dir.join("src").join(format!("{}.rs", macro_name.to_lowercase()));
        fs::write(macro_file, macro_content)?;
    }
    
    // Generate module macro files
    for (module_name, module_macros) in modules {
        let safe_module_name = module_name.replace("::", "_").replace("-", "_");
        let module_macro_name = format!("Mod{}", safe_module_name);
        
        let mut module_content = format!(
            "// Generated module macro for {}\n\
             macro_rules! {} {{\n\
             () => {{\n",
            module_name,
            module_macro_name
        );
        
        for item_macro in &module_macros.items {
            module_content.push_str(&format!("        {}!();\n", item_macro));
        }
        
        module_content.push_str("    };\n}\n");
        
        let module_file = crate_dir.join("src").join(format!("{}.rs", module_macro_name.to_lowercase()));
        fs::write(module_file, module_content)?;
    }
    
    // Generate main lib.rs that includes all macros
    let mut lib_content = String::new();
    lib_content.push_str("// Generated wrapped crate with macro-based items\n\n");
    
    for macro_name in wrapped_items.keys() {
        lib_content.push_str(&format!("include!(\"{}.rs\");\n", macro_name.to_lowercase()));
    }
    
    for module_name in modules.keys() {
        let safe_module_name = module_name.replace("::", "_").replace("-", "_");
        let module_macro_name = format!("Mod{}", safe_module_name);
        lib_content.push_str(&format!("include!(\"{}.rs\");\n", module_macro_name.to_lowercase()));
    }
    
    lib_content.push_str("\n// Execute all items\npub fn execute_all() {\n");
    for module_name in modules.keys() {
        let safe_module_name = module_name.replace("::", "_").replace("-", "_");
        let module_macro_name = format!("Mod{}", safe_module_name);
        lib_content.push_str(&format!("    {}!();\n", module_macro_name));
    }
    lib_content.push_str("}\n");
    
    fs::write(crate_dir.join("src/lib.rs"), lib_content)?;
    
    println!("📝 Generated {} item macros and {} module macros", wrapped_items.len(), modules.len());
    Ok(())
}
