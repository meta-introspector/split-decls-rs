use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use serde_json::Value;
use syn::{parse_file, visit::Visit};

const AUTO_FIX_CACHE_FILE: &str = "autofix_cache.json";

#[derive(Debug)]
struct UnifiedDriver {
    symbol_map: HashMap<String, Value>,
    processed_files: HashMap<String, String>,
    resolved_order: Vec<String>,
    included_crates: HashSet<String>,
    autofix_cache: HashMap<String, String>,
}

impl UnifiedDriver {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔄 Loading symbol map and processed files...");
        
        // Load symbol map
        let compressed_data = fs::read("symbol_map.json.gz")?;
        let mut decoder = flate2::read::GzDecoder::new(&compressed_data[..]);
        let mut symbol_map_content = String::new();
        decoder.read_to_string(&mut symbol_map_content)?;
        let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_map_content)?;
        
        // Load all processed files
        let mut processed_files = HashMap::new();
        if Path::new("submodules/rust/compiler").exists() {
            for entry in fs::read_dir("submodules/rust/compiler")? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let crate_name = entry.file_name().to_string_lossy().to_string();
                    let lib_path = entry.path().join("src/lib.rs");
                    if lib_path.exists() {
                        if let Ok(content) = fs::read_to_string(&lib_path) {
                            processed_files.insert(crate_name, content);
                        }
                    }
                }
            }
        }
        
        // Load autofix cache
        let autofix_cache = load_autofix_cache();
        
        println!("✅ Loaded {} symbols and {} processed files", symbol_map.len(), processed_files.len());
        
        Ok(Self {
            symbol_map,
            processed_files,
            resolved_order: Vec::new(),
            included_crates: HashSet::new(),
            autofix_cache,
        })
    }
    
    pub fn resolve_target_with_deps(&mut self, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Analyzing dependency requirements for: {}", target);
        
        // Load a processed file to trigger mkuse! registrations
        if let Some(entry) = self.symbol_map.get(target) {
            if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                let file_path = format!("processed_{}", source_file.replace("/", "_"));
                if std::path::Path::new(&file_path).exists() {
                    println!("📁 Loading processed file: {}", file_path);
                    // This would trigger mkuse! registrations when compiled
                }
            }
        }
        
        // Print suggestions instead of auto-resolving
        println!("\n📋 SUGGESTIONS:");
        println!("1. Compile a processed file to register mkuse! calls");
        println!("2. Check USE_MATRIX for module->use relationships");
        println!("3. Resolve dependencies systematically from the matrix");
        
        println!("\n🛑 STOPPING - Manual resolution required");
        println!("Next: Use the USE_MATRIX to build precise dependency graph");
        
        Ok(())
    }
    
    fn find_all_dependencies_comprehensive(&mut self, target: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        let mut all_deps = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(target.to_string());
        
        while let Some(current) = queue.pop_front() {
            if all_deps.contains(&current) {
                continue;
            }
            
            all_deps.insert(current.clone());
            
            // Method 1: Get dependencies from symbol map
            if let Some(entry) = self.symbol_map.get(&current) {
                if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                    for dep in deps {
                        if let Some(dep_str) = dep.as_str() {
                            queue.push_back(dep_str.to_string());
                        }
                    }
                }
            } else {
                // Method 2: Try auto-fix for missing symbols
                if let Some(found_symbol) = self.auto_fix_missing_symbol(&current) {
                    println!("🔧 AUTO-FIX: Found {} -> {}", current, found_symbol);
                    queue.push_back(found_symbol);
                    continue;
                }
            }
            
            // Method 3: AST analysis for additional dependencies
            if let Some(source_file) = self.get_source_file(&current) {
                if let Ok(ast_deps) = self.extract_ast_dependencies(&source_file) {
                    for dep in ast_deps {
                        queue.push_back(dep);
                    }
                }
            }
        }
        
        Ok(all_deps)
    }
    
    fn extract_ast_dependencies(&self, source_file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut visitor = DependencyVisitor::new();
        
        if let Ok(ast) = parse_file(source_file) {
            visitor.visit_file(&ast);
        }
        
        Ok(visitor.dependencies)
    }
    
    fn topological_sort(&self, deps: &HashSet<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        for dep in deps {
            if !visited.contains(dep) {
                self.dfs_sort(dep, &mut visited, &mut temp_visited, &mut sorted)?;
            }
        }
        
        sorted.reverse(); // Reverse for correct dependency order
        Ok(sorted)
    }
    
    fn dfs_sort(&self, node: &str, visited: &mut HashSet<String>, temp_visited: &mut HashSet<String>, sorted: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if temp_visited.contains(node) {
            return Ok(()); // Cycle detected, skip
        }
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        // Visit dependencies first
        if let Some(entry) = self.symbol_map.get(node) {
            if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                for dep in deps {
                    if let Some(dep_str) = dep.as_str() {
                        self.dfs_sort(dep_str, visited, temp_visited, sorted)?;
                    }
                }
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        sorted.push(node.to_string());
        
        Ok(())
    }
    
    fn generate_complete_code_with_includes(&mut self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut complete_code = String::new();
        
        // Add header with all features
        complete_code.push_str("#![recursion_limit = \"512\"]\n");
        complete_code.push_str("#![allow(internal_features)]\n");
        complete_code.push_str("#![allow(unused)]\n");
        complete_code.push_str("#![allow(rustc::untranslatable_diagnostic)]\n");
        complete_code.push_str("#![feature(rustc_private)]\n");
        complete_code.push_str("#![feature(core_intrinsics)]\n");
        complete_code.push_str("#![feature(decl_macro)]\n");
        complete_code.push_str("#![feature(panic_backtrace_config)]\n");
        complete_code.push_str("#![feature(panic_update_hook)]\n");
        complete_code.push_str("#![feature(rustdoc_internals)]\n");
        complete_code.push_str("#![feature(try_blocks)]\n\n");
        
        // Add all required extern crates
        let required_crates = self.find_required_crates()?;
        for crate_name in &required_crates {
            complete_code.push_str(&format!("extern crate {};\n", crate_name));
            self.included_crates.insert(crate_name.clone());
        }
        complete_code.push_str("\n");
        
        // Include wrap_types with resolver macros
        complete_code.push_str("// Custom macro to include processed rustc files\n");
        complete_code.push_str("macro_rules! include_rustc {\n");
        complete_code.push_str("    ($crate_name:ident, $file:ident) => {\n");
        complete_code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/\", stringify!($file), \".rs\"));\n");
        complete_code.push_str("    };\n");
        complete_code.push_str("    ($crate_name:ident) => {\n");
        complete_code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/lib.rs\"));\n");
        complete_code.push_str("    };\n");
        complete_code.push_str("}\n\n");
        
        // Include all dependencies in correct order
        let mut included_count = 0;
        for dep in &self.resolved_order {
            if let Some(entry) = self.symbol_map.get(dep) {
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
        
        println!("📁 Included {} dependency files", included_count);
        
        // Add target code
        complete_code.push_str(&format!("// === TARGET: {} ===\n", target));
        complete_code.push_str("fn main() {\n");
        complete_code.push_str(&format!("    println!(\"Executing target: {}\");\n", target));
        complete_code.push_str("}\n");
        
        Ok(complete_code)
    }
    
    fn find_required_crates(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut crates = HashSet::new();
        
        // Standard rustc crates
        let rustc_crates = vec![
            "rustc_driver", "rustc_driver_impl", "rustc_session", "rustc_middle",
            "rustc_ast", "rustc_hir", "rustc_data_structures", "rustc_span",
            "rustc_errors", "rustc_interface", "rustc_codegen_ssa", "rustc_target",
            "rustc_metadata", "rustc_parse", "rustc_expand", "rustc_builtin_macros",
            "rustc_passes", "rustc_mir_build", "rustc_mir_transform", "rustc_mir_dataflow",
            "rustc_const_eval", "rustc_hir_analysis", "rustc_hir_typeck", "rustc_traits",
            "rustc_trait_selection", "rustc_infer", "rustc_borrowck", "rustc_privacy",
            "rustc_resolve", "rustc_lint", "rustc_serialize", "rustc_index", "rustc_macros"
        ];
        
        for crate_name in rustc_crates {
            crates.insert(crate_name.to_string());
        }
        
        Ok(crates.into_iter().collect())
    }
    
    fn get_source_file(&self, symbol: &str) -> Option<String> {
        self.symbol_map.get(symbol)
            .and_then(|entry| entry.get("source_file"))
            .and_then(|s| s.as_str())
            .and_then(|path| fs::read_to_string(format!("submodules/{}", path)).ok())
    }
    
    fn auto_fix_missing_symbol(&mut self, missing_symbol: &str) -> Option<String> {
        // Check cache first
        if let Some(cached_result) = self.autofix_cache.get(missing_symbol) {
            return Some(cached_result.clone());
        }
        
        // Fast partial matching
        let partial_matches: Vec<_> = self.symbol_map.keys()
            .filter(|key| key.contains(missing_symbol) || missing_symbol.contains(*key))
            .take(10)
            .collect();
        
        if let Some(best_match) = partial_matches.first() {
            let result = (*best_match).clone();
            self.autofix_cache.insert(missing_symbol.to_string(), result.clone());
            return Some(result);
        }
        
        None
    }
}

struct DependencyVisitor {
    dependencies: Vec<String>,
}

impl DependencyVisitor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        // Extract use statements as dependencies
        if let syn::UseTree::Path(path) = &node.tree {
            let path_str = path.ident.to_string();
            if !self.dependencies.contains(&path_str) {
                self.dependencies.push(path_str);
            }
        }
        syn::visit::visit_item_use(self, node);
    }
}

fn load_autofix_cache() -> HashMap<String, String> {
    if let Ok(content) = fs::read_to_string(AUTO_FIX_CACHE_FILE) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_autofix_cache(cache: &HashMap<String, String>) {
    if let Ok(cache_data) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(AUTO_FIX_CACHE_FILE, cache_data);
        println!("💾 Saved {} auto-fixes to cache", cache.len());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <target_symbol>", args[0]);
        eprintln!("Example: {} \"rustc_driver::main\"", args[0]);
        std::process::exit(1);
    }
    
    let target = &args[1];
    let mut driver = UnifiedDriver::new()?;
    
    println!("🚀 Starting unified dependency resolution...");
    driver.resolve_target_with_deps(target)?;
    
    // Test compilation
    println!("🔧 Testing compilation...");
    let output = Command::new("cargo")
        .args(&["check", "--lib"])
        .output()?;
    
    if output.status.success() {
        println!("✅ Compilation successful!");
    } else {
        println!("❌ Compilation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
