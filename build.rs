// Auto-generated build.rs using symbol_map.json for complete rustc inclusion
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Building rustc from symbol_map.json...");
    
    // Set required rustc environment variables
    println!("cargo:rustc-env=CFG_RELEASE_CHANNEL=dev");
    println!("cargo:rustc-env=RUSTC_INSTALL_BINDIR=/usr/local/bin");
    
    // Increase recursion limit for complex macros
    println!("cargo:rustc-cfg=feature=\"recursion_limit_256\"");
    
    // Read symbol map
    let symbol_data = fs::read_to_string("symbol_map.json")?;
    let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_data)?;
    
    // Extract all unique source files
    let mut source_files = HashSet::new();
    for (_, symbol_info) in &symbol_map {
        if let Some(source_file) = symbol_info.get("source_file").and_then(|v| v.as_str()) {
            if source_file.contains("/rust/compiler/") && source_file.ends_with(".rs") {
                source_files.insert(source_file.to_string());
            }
        }
    }
    
    // Group files by crate
    let mut crate_files: HashMap<String, Vec<String>> = HashMap::new();
    for file in &source_files {
        if let Some(crate_name) = extract_crate_name(file) {
            crate_files.entry(crate_name).or_insert_with(Vec::new).push(file.clone());
        }
    }
    
    // Generate include file with all submodules
    generate_complete_includes(&crate_files)?;
    
    println!("✅ Generated complete rustc includes from {} files in {} crates", 
             source_files.len(), crate_files.len());
    
    Ok(())
}

fn extract_crate_name_from_extern(extern_decl: &str) -> Option<String> {
    // Extract final crate name from declarations like:
    // "extern crate rustc_middle;" -> "rustc_middle"
    // "extern crate self as rustc_middle;" -> "rustc_middle"
    if let Some(as_pos) = extern_decl.find(" as ") {
        // Handle "extern crate self as name;"
        let after_as = &extern_decl[as_pos + 4..];
        if let Some(semicolon_pos) = after_as.find(';') {
            return Some(after_as[..semicolon_pos].trim().to_string());
        }
    } else if extern_decl.starts_with("extern crate ") {
        // Handle "extern crate name;"
        let after_crate = &extern_decl[13..]; // Skip "extern crate "
        if let Some(semicolon_pos) = after_crate.find(';') {
            return Some(after_crate[..semicolon_pos].trim().to_string());
        }
    }
    None
}

fn extract_crate_name(file_path: &str) -> Option<String> {
    if let Some(compiler_pos) = file_path.find("/rust/compiler/") {
        let after_compiler = &file_path[compiler_pos + "/rust/compiler/".len()..];
        if let Some(slash_pos) = after_compiler.find('/') {
            return Some(after_compiler[..slash_pos].to_string());
        }
    }
    None
}

fn generate_complete_includes(crate_files: &HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut include_file = fs::File::create("src/rustc_complete.rs")?;
    
    writeln!(include_file, "// Auto-generated complete rustc includes from symbol_map.json")?;
    writeln!(include_file, "// All rustc crates and submodules in dependency order")?;
    writeln!(include_file)?;
    
    // Create rustc_complete module structure with common submodules
    writeln!(include_file, "pub mod rustc_complete {{")?;
    writeln!(include_file, "    pub use crate::*;")?;
    writeln!(include_file, "    pub mod ty {{ pub use crate::*; }}")?;
    writeln!(include_file, "    pub mod def_id {{ pub use crate::*; }}")?;
    writeln!(include_file, "    pub mod mir {{ pub use crate::*; }}")?;
    writeln!(include_file, "    pub mod traits {{ pub use crate::*; }}")?;
    writeln!(include_file, "    pub mod query {{ pub use crate::*; }}")?;
    writeln!(include_file, "}}")?;
    writeln!(include_file)?;
    
    // Add collected extern crate declarations at the top
    if let Ok(extern_crates) = fs::read_to_string("src/extern_crates.txt") {
        writeln!(include_file, "// Extern crate declarations moved to crate root")?;
        for line in extern_crates.lines() {
            if !line.trim().is_empty() {
                writeln!(include_file, "{}", line)?;
            }
        }
        writeln!(include_file)?;
    }
    
    // Order crates by dependency (base crates first)
    let base_crates = ["rustc_macros", "rustc_data_structures", "rustc_index", "rustc_span"];
    let mut ordered_crates = Vec::new();
    
    // Add base crates first
    for &base_crate in &base_crates {
        if crate_files.contains_key(base_crate) {
            ordered_crates.push(base_crate.to_string());
        }
    }
    
    // Add remaining crates
    for crate_name in crate_files.keys() {
        if !ordered_crates.contains(crate_name) {
            ordered_crates.push(crate_name.clone());
        }
    }
    
    // Generate includes for each crate
    for (i, crate_name) in ordered_crates.iter().enumerate() {
        if let Some(files) = crate_files.get(crate_name) {
            let unique_module_name = format!("included_{}", crate_name.replace("-", "_"));
            writeln!(include_file, "// {}: {} ({} files)", i + 1, crate_name, files.len())?;
            writeln!(include_file, "pub mod {} {{", unique_module_name)?;
            
            // Include all files in this crate with preprocessing
            for file in files {
                let relative_path = file.replace("../rust/compiler/", "../../rust/compiler/");
                
                // Read, patch, and write preprocessed file
                if let Ok(content) = fs::read_to_string(file) {
                    let mut patched_content = content
                        .replace("//!", "//")  // Fix inner doc comments
                        .replace("/*!", "/*")  // Fix inner block doc comments
                        .replace("#![", "#[");  // Fix inner attributes
                    
                    // Extract extern crate declarations to move to crate root
                    let lines: Vec<&str> = patched_content.lines().collect();
                    let mut extern_crates = Vec::new();
                    let filtered_lines: Vec<&str> = lines.into_iter()
                        .filter(|line| {
                            let trimmed = line.trim();
                            if trimmed.starts_with("extern crate ") && trimmed.ends_with(";") {
                                // Skip problematic crates that don't exist in our environment
                                if !trimmed.contains("mini_core") && !trimmed.contains("alloc_system") {
                                    extern_crates.push(trimmed.to_string());
                                }
                                false // Remove from file content
                            } else {
                                true
                            }
                        })
                        .collect();
                    patched_content = filtered_lines.join("\n");
                    
                    // Store extern crates for later addition to crate root
                    if !extern_crates.is_empty() {
                        let extern_crates_file = "src/extern_crates.txt";
                        let existing = fs::read_to_string(extern_crates_file).unwrap_or_default();
                        let mut crate_names: HashSet<String> = HashSet::new();
                        let mut deduplicated_crates = Vec::new();
                        
                        // Process existing crates
                        for line in existing.lines() {
                            if let Some(name) = extract_crate_name_from_extern(line) {
                                if crate_names.insert(name) {
                                    deduplicated_crates.push(line.to_string());
                                }
                            }
                        }
                        
                        // Process new crates
                        for crate_decl in extern_crates {
                            if let Some(name) = extract_crate_name_from_extern(&crate_decl) {
                                if crate_names.insert(name) {
                                    deduplicated_crates.push(crate_decl);
                                }
                            }
                        }
                        
                        fs::write(extern_crates_file, deduplicated_crates.join("\n"))?;
                    }
                    
                    // Fix crate paths but skip lines with macro variables
                    let lines: Vec<&str> = patched_content.lines().collect();
                    let fixed_lines: Vec<String> = lines.into_iter()
                        .map(|line| {
                            // Skip crate:: replacement if line contains macro variables
                            if line.contains("$") {
                                line.to_string()
                            } else {
                                line.replace("crate::", &format!("{}::", crate_name.replace("-", "_")))
                            }
                        })
                        .collect();
                    // Fix rustc crate imports to use local modules
                    patched_content = patched_content.replace("rustc_middle::", "crate::rustc_middle::");
                    patched_content = patched_content.replace("rustc_hir::", "crate::rustc_hir::");
                    patched_content = patched_content.replace("rustc_ast::", "crate::rustc_ast::");
                    patched_content = patched_content.replace("rustc_data_structures::", "crate::rustc_data_structures::");
                    patched_content = patched_content.replace("rustc_session::", "crate::rustc_session::");
                    patched_content = patched_content.replace("rustc_span::", "crate::rustc_span::");
                    patched_content = patched_content.replace("rustc_errors::", "crate::rustc_errors::");
                    patched_content = patched_content.replace("rustc_infer::", "crate::rustc_infer::");
                    patched_content = patched_content.replace("rustc_trait_selection::", "crate::rustc_trait_selection::");
                    
                    // Fix specific import patterns that are problematic
                    patched_content = patched_content.replace("use crate::rustc_span::", "use crate::rustc_complete::");
                    patched_content = patched_content.replace("use crate::rustc_ast::", "use crate::rustc_complete::");
                    patched_content = patched_content.replace("use crate::rustc_middle::", "use crate::rustc_complete::");
                    patched_content = patched_content.replace("use crate::rustc_errors::", "use crate::rustc_complete::");
                    patched_content = patched_content.replace("use crate::rustc_session::", "use crate::rustc_complete::");
                    patched_content = patched_content.replace("use crate::rustc_hir::", "use crate::rustc_complete::");
                    
                    // Remove proc_macro attributes since we're not a proc-macro crate
                    patched_content = patched_content.replace("#[proc_macro]", "// #[proc_macro] - removed");
                    patched_content = patched_content.replace("#[proc_macro_derive", "// #[proc_macro_derive");
                    
                    // Fix unsafe attributes for Rust 2024 edition
                    patched_content = patched_content.replace("#[no_mangle]", "#[unsafe(no_mangle)]");
                    patched_content = patched_content.replace("#[export_name", "#[unsafe(export_name");
                    patched_content = patched_content.replace("#[link_section", "#[unsafe(link_section");
                    
                    // Fix extern blocks to be unsafe for Rust 2024 edition
                    patched_content = patched_content.replace("extern \"C\" {", "unsafe extern \"C\" {");
                    patched_content = patched_content.replace("extern \"system\" {", "unsafe extern \"system\" {");
                    patched_content = patched_content.replace("extern {", "unsafe extern {");
                    
                    // Fix macro name conflicts by making them crate-specific
                    if crate_name == "rustc_hir" && patched_content.contains("macro_rules! arena_types") {
                        patched_content = patched_content.replace("macro_rules! arena_types", "macro_rules! hir_arena_types");
                    }
                    
                    // Fix internal module path references
                    if crate_name == "rustc_metadata" && patched_content.contains("rustc_metadata::rmeta") {
                        patched_content = patched_content.replace("rustc_metadata::rmeta", "rustc_metadata_rmeta");
                    }
                    // Fix visibility paths - handle both patterns
                    if crate_name == "rustc_metadata" {
                        if patched_content.contains("pub(in rustc_metadata::rmeta)") {
                            patched_content = patched_content.replace("pub(in rustc_metadata::rmeta)", "pub");
                        }
                        if patched_content.contains("pub(in crate::rmeta)") {
                            patched_content = patched_content.replace("pub(in crate::rmeta)", "pub");
                        }
                    }
                    if crate_name == "rustc_mir_build" && patched_content.contains("rustc_mir_build::builder") {
                        patched_content = patched_content.replace("rustc_mir_build::builder", "rustc_mir_build_builder");
                    }
                    if crate_name == "rustc_next_trait_solver" && patched_content.contains("rustc_next_trait_solver::solve") {
                        patched_content = patched_content.replace("rustc_next_trait_solver::solve", "rustc_next_trait_solver_solve");
                    }
                    if crate_name == "rustc_borrowck" && patched_content.contains("rustc_borrowck::diagnostics") {
                        patched_content = patched_content.replace("rustc_borrowck::diagnostics", "rustc_borrowck_diagnostics");
                    }
                    
                    // Add more specific visibility patches
                    if crate_name == "rustc_borrowck" && patched_content.contains("pub(in crate::diagnostics)") {
                        patched_content = patched_content.replace("pub(in crate::diagnostics)", "pub");
                    }
                    if crate_name == "rustc_next_trait_solver" && patched_content.contains("pub(in crate::solve)") {
                        patched_content = patched_content.replace("pub(in crate::solve)", "pub");
                    }
                    if crate_name == "rustc_mir_build" && patched_content.contains("pub(in crate::builder)") {
                        patched_content = patched_content.replace("pub(in crate::builder)", "pub");
                    }
                    
                    // Remove mod declarations since we're including all files directly
                    let lines: Vec<&str> = patched_content.lines().collect();
                    let mut filtered_lines = Vec::new();
                    let mut i = 0;
                    
                    while i < lines.len() {
                        let line = lines[i];
                        let trimmed = line.trim();
                        
                        // Check if this is a mod declaration to remove (various patterns)
                        if (trimmed.contains("mod ") && trimmed.contains(";")) {
                            // This catches: mod foo;, pub mod foo;, pub(super) mod foo;, mod foo; // comment
                            i += 1;
                            continue;
                        }
                        
                        // Check if this is an orphaned #[cfg(test)] attribute
                        if trimmed == "#[cfg(test)]" && i + 1 < lines.len() {
                            let next_line = lines[i + 1].trim();
                            if (next_line.starts_with("mod ") && next_line.ends_with(";")) ||
                               (next_line.starts_with("pub mod ") && next_line.ends_with(";")) {
                                // Skip both the attribute and the mod declaration
                                i += 2;
                                continue;
                            }
                        }
                        
                        filtered_lines.push(line);
                        i += 1;
                    }
                    
                    patched_content = filtered_lines.join("\n");
                    
                    // Handle internal include! statements by copying referenced files
                    if patched_content.contains("include!(\"../build_system/shared_utils.rs\")") {
                        let shared_utils_path = file.replace("/cargo-clif.rs", "/../build_system/shared_utils.rs");
                        if let Ok(shared_utils_content) = fs::read_to_string(&shared_utils_path) {
                            // Replace include! with actual content
                            patched_content = patched_content.replace(
                                "include!(\"../build_system/shared_utils.rs\");",
                                &format!("// Inlined shared_utils.rs\n{}", shared_utils_content)
                            );
                        }
                    }
                    
                    // Handle .data file includes
                    if patched_content.contains("include!(\"list_and_v1.rs.data\")") {
                        let data_file_path = file.replace("/mod.rs", "/list_and_v1.rs.data");
                        if let Ok(data_content) = fs::read_to_string(&data_file_path) {
                            // Copy data file to src/
                            fs::write("src/list_and_v1.rs.data", data_content)?;
                        }
                    }
                    
                    // Handle include_str! text files
                    if patched_content.contains("include_str!(\"usage.txt\")") {
                        let usage_file_path = file.replace("/main.rs", "/usage.txt");
                        if let Ok(usage_content) = fs::read_to_string(&usage_file_path) {
                            // Copy usage file to src/
                            fs::write("src/usage.txt", usage_content)?;
                        }
                    }
                    
                    // Handle data/mod.rs includes - skip to avoid duplicates
                    if patched_content.contains("include!(\"data/mod.rs\")") {
                        // Replace the include with a comment since we'll handle this manually
                        patched_content = patched_content.replace(
                            "include!(\"data/mod.rs\");",
                            "// Skipped data/mod.rs - handled manually to avoid duplicates"
                        );
                    }
                    
                    let file_name = file.split('/').last().unwrap_or("unknown").replace(".rs", "");
                    // Make processed filename unique by including more path info
                    let path_parts: Vec<&str> = file.split('/').collect();
                    let unique_file_name = if path_parts.len() >= 4 {
                        let dirs = &path_parts[path_parts.len()-3..path_parts.len()-1];
                        format!("{}_{}", dirs.join("_").replace("-", "_").replace(".", "_"), file_name)
                    } else if path_parts.len() >= 3 {
                        let dir = path_parts[path_parts.len()-2];
                        format!("{}_{}", dir.replace("-", "_").replace(".", "_"), file_name)
                    } else {
                        file_name.clone()
                    };
                    
                    let processed_path = format!("src/processed_{}_{}.rs", 
                        crate_name.replace("-", "_"), unique_file_name);
                    
                    fs::write(&processed_path, patched_content)?;
                    
                    // Wrap each include in its own unique module (use full path for uniqueness)
                    let path_parts: Vec<&str> = file.split('/').collect();
                    let unique_name = if path_parts.len() >= 4 {
                        // Use multiple directory levels + filename for maximum uniqueness
                        let dirs = &path_parts[path_parts.len()-3..path_parts.len()-1];
                        format!("{}_{}_{}", 
                            crate_name.replace("-", "_"), 
                            dirs.join("_").replace("-", "_").replace(".", "_"),
                            file_name.replace("-", "_").replace(".", "_"))
                    } else if path_parts.len() >= 3 {
                        // Use directory + filename for uniqueness
                        let dir = path_parts[path_parts.len()-2];
                        format!("{}_{}_{}", 
                            crate_name.replace("-", "_"), 
                            dir.replace("-", "_").replace(".", "_"),
                            file_name.replace("-", "_").replace(".", "_"))
                    } else {
                        format!("{}_{}", 
                            crate_name.replace("-", "_"), 
                            file_name.replace("-", "_").replace(".", "_"))
                    };
                    writeln!(include_file, "    // Source: {}", file)?;
                    writeln!(include_file, "    pub mod {} {{", unique_name)?;
                    writeln!(include_file, "        include!(\"{}\");", processed_path.replace("src/", ""))?;
                    writeln!(include_file, "    }}")?;
                } else {
                    writeln!(include_file, "    // File not found: {}", relative_path)?;
                }
            }
            
            writeln!(include_file, "}}")?;
            writeln!(include_file)?;
        }
    }
    
    println!("✅ Generated src/rustc_complete.rs with {} patched crates", ordered_crates.len());
    Ok(())
}
