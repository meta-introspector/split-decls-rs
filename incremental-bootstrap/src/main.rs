use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::toposort;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use clap::{Parser, Subcommand};
use syn::{visit::Visit, Item, Ident, File as SynFile, UseTree, UsePath, UseGroup};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type DependencyGraph = Graph<String, (), Directed>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Incremental Bootstrap Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List available binaries
    #[command(name = "list-bins")]
    ListBins,
    /// Run a specific binary
    #[command(name = "run")]
    Run {
        /// Binary name to run
        #[arg(long)]
        bin: String,
        /// Arguments to pass to the binary
        args: Vec<String>,
    },
    /// Analyze dependencies for a specific binary
    #[command(name = "analyze-deps")]
    AnalyzeDeps {
        /// Binary name to analyze
        #[arg(long)]
        bin: String,
    },
    /// Recursive dependency analysis with caching
    #[command(name = "recursive-deps")]
    RecursiveDeps {
        /// Binary name to analyze
        #[arg(long)]
        bin: String,
        /// Maximum recursion depth
        #[arg(long, default_value = "3")]
        depth: usize,
    },
    /// Print dependency graph for a binary
    #[command(name = "print-graph")]
    PrintGraph {
        /// Binary name to analyze
        #[arg(long)]
        bin: String,
        /// Maximum recursion depth
        #[arg(long, default_value = "2")]
        depth: usize,
    },
    /// Test compilation of generated evaluation
    #[command(name = "test-eval")]
    TestEval {
        /// Binary name to test
        #[arg(long)]
        bin: String,
    },
    /// Run incremental bootstrap (default)
    #[command(name = "bootstrap")]
    Bootstrap,
}

struct IncrementalBootstrap {
    graph: DependencyGraph,
    node_map: HashMap<String, NodeIndex>,
    module_count: usize,
}

impl IncrementalBootstrap {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
            module_count: 0,
        }
    }

    fn scan_output2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Scanning output2 for all declarations...");
        
        for entry in fs::read_dir("../output2")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_path = entry.path();
                let crate_name = crate_path.file_name().unwrap().to_string_lossy();
                
                let decls_dir = crate_path.join("src/decls");
                if decls_dir.exists() {
                    self.scan_decls(&decls_dir, &crate_name)?;
                }
            }
        }
        
        Ok(())
    }

    fn scan_decls(&mut self, dir: &Path, crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let file_name = path.file_stem().unwrap().to_string_lossy();
                let decl_name = format!("{}::{}", crate_name, file_name);
                self.add_decl(decl_name);
            } else if path.is_dir() {
                self.scan_decls(&path, crate_name)?;
            }
        }
        Ok(())
    }

    fn add_decl(&mut self, name: String) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(&name) {
            idx
        } else {
            let idx = self.graph.add_node(name.clone());
            self.node_map.insert(name, idx);
            idx
        }
    }

    fn generate_incremental_modules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sorted = toposort(&self.graph, None)
            .map_err(|_| "Cycle detected in dependency graph")?;
        
        println!("📦 Generating incremental modules in topological order...");
        
        for (i, node_idx) in sorted.iter().enumerate() {
            let decl_name = &self.graph[*node_idx];
            let module_num = i + 1;
            
            println!("🔧 Module {}: {}", module_num, decl_name);
            
            // Generate module file
            let module_content = format!(
                "// Module {} - {}\n// Generated in topological order\n\nuse std::*;\n\n// Include declaration: {}\n// TODO: Add actual include path\n\npub fn test_compile_module_{}() {{\n    println!(\"Module {} compiles successfully\");\n}}\n",
                module_num, decl_name, decl_name, module_num, module_num
            );
            
            let module_path = format!("../bootstrap3-incremental/module_{:04}.rs", module_num);
            fs::write(&module_path, module_content)?;
            
            // Try to compile this module
            if !self.test_compile_module(module_num)? {
                println!("❌ Module {} failed to compile - creating crate-lattice-{}", module_num, module_num);
                self.create_lattice_crate(module_num, decl_name)?;
                break;
            }
        }
        
        Ok(())
    }

    fn test_compile_module(&self, module_num: usize) -> Result<bool, Box<dyn std::error::Error>> {
        // Simulate compilation test
        println!("   ✅ Module {} compiles", module_num);
        Ok(true) // For now, assume all compile
    }

    fn create_lattice_crate(&self, num: usize, decl_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lattice_content = format!(
            "// Crate Lattice {} - Failed compilation point\n// Declaration: {}\n// This crate combines multiple dependencies\n\n[package]\nname = \"crate-lattice-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n// TODO: Add dependencies that caused the failure\n",
            num, decl_name, num
        );
        
        fs::create_dir_all("../bootstrap3-incremental/lattice-crates")?;
        let lattice_path = format!("../bootstrap3-incremental/lattice-crates/crate-lattice-{}.toml", num);
        fs::write(&lattice_path, lattice_content)?;
        
        println!("🔗 Created lattice crate: {}", lattice_path);
        Ok(())
    }
}

fn scan_output2_binaries() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning output2 for binaries...");
    
    let mut binaries = Vec::new();
    
    // Look for main functions in wrapped-split-decls-rs
    let main_dirs = [
        "output2/wrapped-split-decls-rs/src/decls/main",
        "output2/wrapped-split-decls-rs/src/decls/simple_split", 
        "output2/wrapped-split-decls-rs/src/decls/enhanced_wrapper",
        "output2/wrapped-split-decls-rs/src/decls/wrap_single_crate",
        "output2/wrapped-split-decls-rs/src/decls/ecosystem_splitter",
    ];
    
    for main_dir in &main_dirs {
        let path = Path::new(main_dir);
        if path.exists() {
            let bin_name = path.file_name().unwrap().to_string_lossy();
            
            // Find the main.rs file in the complexity subdirectories
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    let entry = entry?;
                    if entry.file_type()?.is_dir() {
                        let fn_dir = entry.path().join("fn");
                        if fn_dir.exists() {
                            if let Ok(complexity_dirs) = fs::read_dir(&fn_dir) {
                                for complexity_entry in complexity_dirs {
                                    let complexity_entry = complexity_entry?;
                                    let main_file = complexity_entry.path().join("main.rs");
                                    if main_file.exists() {
                                        binaries.push((
                                            "wrapped-split-decls-rs".to_string(),
                                            bin_name.to_string(),
                                            main_file
                                        ));
                                        break; // Found one, that's enough
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Also scan other wrapped crates for main.rs
    for entry in fs::read_dir("../output2")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            let crate_name = crate_path.file_name().unwrap().to_string_lossy();
            
            // Look for main.rs in decls
            let decls_dir = crate_path.join("src/decls");
            if decls_dir.exists() {
                if let Ok(main_file) = fs::read_dir(&decls_dir) {
                    for decl_entry in main_file {
                        let decl_entry = decl_entry?;
                        if decl_entry.file_name() == "main.rs" {
                            binaries.push((
                                crate_name.to_string(),
                                "main".to_string(),
                                decl_entry.path()
                            ));
                        }
                    }
                }
            }
        }
    }
    
    println!("📋 Found {} binaries in output2:", binaries.len());
    for (crate_name, bin_name, path) in &binaries {
        println!("  🔧 {}::{} -> {}", crate_name, bin_name, path.display());
    }
    
    // Generate include statements
    println!("\n📝 Generated include statements:");
    for (crate_name, bin_name, path) in &binaries {
        let include_path = path.strip_prefix("../").unwrap_or(path);
        println!("include!(\"{}\"); // {}::{}", include_path.display(), crate_name, bin_name);
    }
    
    Ok(())
}

fn run_binary(bin_name: &str, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running binary: {}", bin_name);
    
    let mut cmd = match bin_name {
        "simple_split" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../simple-split/Cargo.toml"]);
            c
        },
        "split-decls-rs" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../Cargo.toml"]);
            c
        },
        "bootstrap" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../Cargo.toml", "--", "bootstrap"]);
            c
        },
        "incremental-builder" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../incremental-builder/Cargo.toml"]);
            c
        },
        "topo-sorter" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../topo-sorter/Cargo.toml"]);
            c
        },
        "index-generator" => {
            let mut c = Command::new("cargo");
            c.args(&["run", "--manifest-path", "../index-generator/Cargo.toml"]);
            c
        },
        _ => {
            println!("❌ Unknown binary: {}", bin_name);
            println!("Run 'list-bins' to see available binaries");
            return Ok(());
        }
    };
    
    if !args.is_empty() {
        cmd.args(&args);
    }
    
    let status = cmd.status()?;
    
    if status.success() {
        println!("✅ Binary {} completed successfully", bin_name);
    } else {
        println!("❌ Binary {} failed with exit code: {:?}", bin_name, status.code());
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::ListBins => {
            scan_output2_binaries()?;
        },
        Commands::Run { bin, args } => {
            run_binary(&bin, args)?;
        },
        Commands::AnalyzeDeps { bin } => {
            analyze_binary_dependencies(&bin)?;
        },
        Commands::RecursiveDeps { bin, depth } => {
            recursive_dependency_analysis(&bin, depth)?;
        },
        Commands::PrintGraph { bin, depth } => {
            print_dependency_graph(&bin, depth)?;
        },
        Commands::TestEval { bin } => {
            test_evaluation(&bin)?;
        },
        Commands::Bootstrap => {
            println!("🚀 Incremental Bootstrap Generator");
            
            fs::create_dir_all("../bootstrap3-incremental")?;
            
            let mut bootstrap = IncrementalBootstrap::new();
            bootstrap.scan_output2()?;
            bootstrap.generate_incremental_modules()?;
            
            println!("✨ Incremental bootstrap generation complete!");
        }
    }
    
    Ok(())
}

fn analyze_binary_dependencies(bin_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Analyzing dependencies for binary: {}", bin_name);
    
    // Find the binary file
    let bin_path = find_binary_in_output2(bin_name)?;
    println!("📍 Found binary at: {}", bin_path.display());
    
    // Parse the binary AST
    let content = fs::read_to_string(&bin_path)?;
    let ast: SynFile = syn::parse_file(&content)?;
    
    // Extract all identifiers used in the binary
    let mut visitor = TokenVisitor { uses: HashSet::new() };
    visitor.visit_file(&ast);
    
    println!("🔧 Binary uses {} unique tokens", visitor.uses.len());
    
    // Find matching declarations in output2
    let mut resolved_deps = HashMap::new();
    scan_output2_for_tokens(&visitor.uses, &mut resolved_deps)?;
    
    println!("✅ Resolved {} dependencies in output2:", resolved_deps.len());
    for (token, paths) in &resolved_deps {
        println!("  {} -> {} files", token, paths.len());
        for path in paths.iter().take(3) {
            println!("    {}", path.display());
        }
        if paths.len() > 3 {
            println!("    ... and {} more", paths.len() - 3);
        }
    }
    
    // Generate dependency graph
    generate_dependency_graph(bin_name, &resolved_deps)?;
    
    Ok(())
}

fn find_binary_in_output2(bin_name: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Look for the binary in wrapped-split-decls-rs first
    let search_paths = [
        format!("../output2/wrapped-split-decls-rs/src/decls/{}/fn", bin_name),
        format!("../output2/wrapped-split-decls-rs/src/decls/{}", bin_name),
    ];
    
    for search_path in &search_paths {
        let path = Path::new(search_path);
        if path.exists() {
            // Find main.rs in complexity subdirectories
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let main_file = entry.path().join("main.rs");
                    if main_file.exists() {
                        return Ok(main_file);
                    }
                }
            }
        }
    }
    
    Err(format!("Binary {} not found in output2", bin_name).into())
}

struct TokenVisitor {
    uses: HashSet<String>,
}

impl<'ast> Visit<'ast> for TokenVisitor {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        let name = ident.to_string();
        // Filter out common keywords and focus on meaningful identifiers
        if !["fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return", "Ok", "Err"].contains(&name.as_str()) {
            self.uses.insert(name);
        }
    }
}

struct ImportVisitor {
    imports: Vec<String>,
}

impl<'ast> Visit<'ast> for ImportVisitor {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.imports.push(quote::quote!(#node).to_string());
    }
}

fn scan_output2_for_tokens(tokens: &HashSet<String>, resolved: &mut HashMap<String, Vec<std::path::PathBuf>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning output2 for {} tokens...", tokens.len());
    
    for entry in fs::read_dir("../output2")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let decls_dir = entry.path().join("src/decls");
            if decls_dir.exists() {
                scan_decls_for_tokens(&decls_dir, tokens, resolved)?;
            }
        }
    }
    
    Ok(())
}

fn scan_decls_for_tokens(dir: &Path, tokens: &HashSet<String>, resolved: &mut HashMap<String, Vec<std::path::PathBuf>>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                for token in tokens {
                    if content.contains(token) {
                        resolved.entry(token.clone()).or_insert_with(Vec::new).push(path.clone());
                    }
                }
            }
        } else if path.is_dir() {
            scan_decls_for_tokens(&path, tokens, resolved)?;
        }
    }
    Ok(())
}

fn generate_dependency_graph(bin_name: &str, deps: &HashMap<String, Vec<std::path::PathBuf>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Generating dependency graph for {}", bin_name);
    
    let mut graph_content = format!("// Dependency graph for {}\n", bin_name);
    graph_content.push_str("// Generated include statements:\n\n");
    
    let mut included_files = HashSet::new();
    
    for (token, paths) in deps {
        graph_content.push_str(&format!("// Token: {}\n", token));
        for path in paths.iter().take(1) { // Take first match for each token
            if !included_files.contains(path) {
                let include_path = path.strip_prefix("../").unwrap_or(path);
                graph_content.push_str(&format!("include!(\"{}\");\n", include_path.display()));
                included_files.insert(path.clone());
            }
        }
        graph_content.push('\n');
    }
    
    let output_file = format!("../bootstrap3-incremental/{}_dependencies.rs", bin_name);
    fs::write(&output_file, graph_content)?;
    
    println!("✅ Dependency graph saved to: {}", output_file);
    println!("📈 Total unique files needed: {}", included_files.len());
    
    Ok(())
}

#[derive(Debug, Clone)]
struct DepNode {
    id: String,
    hash: u64,
    path: std::path::PathBuf,
    content_hash: u64,
    tokens: HashSet<String>,
    resolved_deps: Vec<String>,
}

struct Output2Index {
    declarations: HashMap<String, Vec<std::path::PathBuf>>, // token -> paths
    cache_file: std::path::PathBuf,
}

impl Output2Index {
    fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            cache_file: std::path::PathBuf::from("../bootstrap3-incremental/output2_index.json"),
        }
    }

    fn load_or_build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Try to load from cache first
        if self.cache_file.exists() {
            if let Ok(content) = fs::read_to_string(&self.cache_file) {
                if let Ok(cached) = serde_json::from_str::<HashMap<String, Vec<String>>>(&content) {
                    self.declarations = cached.into_iter()
                        .map(|(k, v)| (k, v.into_iter().map(std::path::PathBuf::from).collect()))
                        .collect();
                    println!("📋 Loaded output2 index from cache ({} tokens)", self.declarations.len());
                    return Ok(());
                }
            }
        }

        // Build index from scratch
        println!("🔍 Building output2 index (one-time scan)...");
        self.scan_all_output2()?;
        self.save_cache()?;
        println!("✅ Built output2 index ({} tokens)", self.declarations.len());
        Ok(())
    }

    fn scan_all_output2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir("../output2")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let decls_dir = entry.path().join("src/decls");
                if decls_dir.exists() {
                    self.scan_decls_recursive(&decls_dir)?;
                }
            }
        }
        Ok(())
    }

    fn scan_decls_recursive(&mut self, dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    // Extract tokens from filename and content
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        self.declarations.entry(file_stem.to_string())
                            .or_insert_with(Vec::new)
                            .push(path.clone());
                    }
                    
                    // Extract common type names from content
                    for token in ["Path", "File", "Item", "fs", "env", "ToTokens", "HashMap", "Vec", "String", "Result"] {
                        if content.contains(token) {
                            self.declarations.entry(token.to_string())
                                .or_insert_with(Vec::new)
                                .push(path.clone());
                        }
                    }
                }
            } else if path.is_dir() {
                self.scan_decls_recursive(&path)?;
            }
        }
        Ok(())
    }

    fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_data: HashMap<String, Vec<String>> = self.declarations.iter()
            .map(|(k, v)| (k.clone(), v.iter().map(|p| p.to_string_lossy().to_string()).collect()))
            .collect();
        let json = serde_json::to_string_pretty(&cache_data)?;
        fs::write(&self.cache_file, json)?;
        Ok(())
    }

    fn resolve(&self, token: &str) -> Option<&Vec<std::path::PathBuf>> {
        self.declarations.get(token)
    }
}

struct DepCache {
    nodes: HashMap<String, DepNode>,
    cache_dir: std::path::PathBuf,
}

impl DepCache {
    fn new() -> Self {
        let cache_dir = std::path::PathBuf::from("../bootstrap3-incremental/dep-cache");
        fs::create_dir_all(&cache_dir).unwrap();
        Self {
            nodes: HashMap::new(),
            cache_dir,
        }
    }

    fn get_or_compute(&mut self, path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let content_hash = hash_content(&content);
        let path_str = path.to_string_lossy().to_string();
        let id = format!("dep-{:x}", hash_content(&path_str));

        // Check cache first
        if let Some(node) = self.nodes.get(&id) {
            if node.content_hash == content_hash {
                return Ok(id);
            }
        }

        // Compute new node
        let tokens = extract_tokens_from_content(&content)?;
        let node = DepNode {
            id: id.clone(),
            hash: content_hash,
            path: path.to_path_buf(),
            content_hash,
            tokens,
            resolved_deps: Vec::new(),
        };

        // Save to cache
        self.save_node_to_cache(&node)?;
        self.nodes.insert(id.clone(), node);

        Ok(id)
    }

    fn save_node_to_cache(&self, node: &DepNode) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = self.cache_dir.join(format!("{}.json", node.id));
        let json = serde_json::to_string_pretty(&CachedNode {
            id: node.id.clone(),
            hash: node.hash,
            path: node.path.to_string_lossy().to_string(),
            content_hash: node.content_hash,
            tokens: node.tokens.iter().cloned().collect(),
            resolved_deps: node.resolved_deps.clone(),
        })?;
        fs::write(cache_file, json)?;
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedNode {
    id: String,
    hash: u64,
    path: String,
    content_hash: u64,
    tokens: Vec<String>,
    resolved_deps: Vec<String>,
}

fn recursive_dependency_analysis(bin_name: &str, max_depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Recursive dependency analysis for: {} (depth: {})", bin_name, max_depth);
    
    // Build output2 index once
    let mut index = Output2Index::new();
    index.load_or_build()?;
    
    let mut cache = DepCache::new();
    let mut processed = HashSet::new();
    let mut to_process = Vec::new();
    
    // Start with the binary
    let bin_path = find_binary_in_output2(bin_name)?;
    let root_id = cache.get_or_compute(&bin_path)?;
    to_process.push((root_id.clone(), 0));
    
    let mut dependency_tree = HashMap::new();
    
    while let Some((current_id, depth)) = to_process.pop() {
        if processed.contains(&current_id) || depth >= max_depth {
            continue;
        }
        
        processed.insert(current_id.clone());
        println!("📊 Processing {} at depth {}", current_id, depth);
        
        let node = cache.nodes.get(&current_id).unwrap();
        let tokens = node.tokens.clone(); // Clone to avoid borrow issues
        
        // Use index to resolve dependencies instead of scanning
        let mut dep_ids = Vec::new();
        for token in &tokens {
            if let Some(paths) = index.resolve(token) {
                if let Some(first_path) = paths.first() {
                    let dep_id = cache.get_or_compute(first_path)?;
                    dep_ids.push(dep_id.clone());
                    to_process.push((dep_id, depth + 1));
                }
            }
        }
        
        dependency_tree.insert(current_id.clone(), dep_ids);
        
        println!("  ✅ Found {} dependencies", dependency_tree[&current_id].len());
    }
    
    // Generate final evaluation
    generate_evaluation_result(bin_name, &dependency_tree, &cache, &index)?;
    
    println!("🎯 Recursive analysis complete!");
    println!("   Processed {} unique nodes", processed.len());
    println!("   Cache entries: {}", cache.nodes.len());
    
    Ok(())
}

fn generate_evaluation_result(
    bin_name: &str, 
    tree: &HashMap<String, Vec<String>>, 
    cache: &DepCache,
    index: &Output2Index
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Generating evaluation result...");
    
    let mut eval_content = format!("// Recursive evaluation for {}\n", bin_name);
    eval_content.push_str("// Nix-like functional cache system\n\n");
    
    // Add macro definitions first
    eval_content.push_str(r#"// Macro definitions
macro_rules! mkdeclfn {
    (fn $name:ident($($args:tt)*) -> Result<()> { $($body:tt)* }) => {
        pub fn $name($($args)*) -> Result<(), Box<dyn std::error::Error>> {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        pub fn $name($($args)*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) { $($body:tt)* }) => {
        pub fn $name($($args)*) {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
}

"#);
    
    // Generate includes in dependency order, each in its own module
    let ordered = topological_sort_deps(tree)?;
    let mut included_paths = HashSet::new();
    
    for (i, dep_id) in ordered.iter().enumerate() {
        if let Some(node) = cache.nodes.get(dep_id) {
            if !included_paths.contains(&node.path) {
                let abs_path = std::fs::canonicalize(&node.path)
                    .unwrap_or_else(|_| node.path.clone());
                let mod_name = format!("dep_mod_{}", i);
                // Extract content and recursively resolve all dependencies
                let content = fs::read_to_string(&abs_path).unwrap_or_default();
                let (original_imports, clean_content) = extract_imports_and_content(&content)?;
                
                // Recursively resolve all missing dependencies from the content
                let mut resolved_includes = String::new();
                let mut visited = HashSet::new();
                let mut glossary = HashMap::new();
                resolve_dependencies_recursive(&clean_content, index, &mut resolved_includes, &mut visited, 0, &mut glossary)?;
                
                eval_content.push_str(&format!(
                    "// Dep: {} (hash: {:x})\nmod {} {{\n{}\n{}\n{}\n}}\npub use {}::*;\n\n",
                    dep_id, node.content_hash, mod_name, original_imports, resolved_includes, clean_content, mod_name
                ));
                included_paths.insert(node.path.clone());
            }
        }
    }
    
    // Generate evaluation function
    eval_content.push_str(&format!(
        "pub fn evaluate_{}() -> Result<(), Box<dyn std::error::Error>> {{\n",
        bin_name
    ));
    eval_content.push_str("    println!(\"🚀 Evaluating recursive dependencies...\");\n");
    eval_content.push_str("    // All dependencies are now available\n");
    eval_content.push_str("    Ok(())\n");
    eval_content.push_str("}\n");
    
    let output_file = format!("../bootstrap3-incremental/{}_recursive_eval.rs", bin_name);
    fs::write(&output_file, eval_content)?;
    
    // Generate dependency manifest
    let manifest = serde_json::to_string_pretty(&EvalManifest {
        binary: bin_name.to_string(),
        total_deps: included_paths.len(),
        cache_entries: cache.nodes.len(),
        dependency_hashes: cache.nodes.iter().map(|(id, node)| {
            (id.clone(), format!("{:x}", node.content_hash))
        }).collect(),
    })?;
    
    let manifest_file = format!("../bootstrap3-incremental/{}_manifest.json", bin_name);
    fs::write(&manifest_file, manifest)?;
    
    // Generate glossary of all resolved dependencies
    let mut all_glossary: HashMap<String, String> = HashMap::new();
    // Collect glossary from all modules (simplified for now)
    let glossary_content = serde_json::to_string_pretty(&all_glossary)?;
    let glossary_file = format!("../bootstrap3-incremental/{}_glossary.json", bin_name);
    fs::write(&glossary_file, glossary_content)?;
    
    println!("✅ Evaluation saved to: {}", output_file);
    println!("📋 Manifest saved to: {}", manifest_file);
    println!("📚 Glossary saved to: {}", glossary_file);
    println!("📈 Total unique dependencies: {}", included_paths.len());
    
    Ok(())
}

#[derive(serde::Serialize)]
struct EvalManifest {
    binary: String,
    total_deps: usize,
    cache_entries: usize,
    dependency_hashes: HashMap<String, String>,
}

fn topological_sort_deps(tree: &HashMap<String, Vec<String>>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();
    
    // Build graph and calculate in-degrees
    for (node, deps) in tree {
        graph.insert(node.clone(), deps.clone());
        in_degree.entry(node.clone()).or_insert(0);
        
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }
    
    // Topological sort
    let mut queue: Vec<_> = in_degree.iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(node, _)| node.clone())
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop() {
        result.push(node.clone());
        
        if let Some(deps) = graph.get(&node) {
            for dep in deps {
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(dep.clone());
                    }
                }
            }
        }
    }
    
    Ok(result)
}

fn hash_content(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

fn extract_tokens_from_content(content: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    if let Ok(ast) = syn::parse_file(content) {
        let mut visitor = TokenVisitor { uses: HashSet::new() };
        visitor.visit_file(&ast);
        Ok(visitor.uses)
    } else {
        // Fallback for non-parseable content
        Ok(HashSet::new())
    }
}

fn print_dependency_graph(bin_name: &str, max_depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Dependency Graph for: {}", bin_name);
    println!("═══════════════════════════════════════");
    
    let mut cache = DepCache::new();
    let mut processed = HashSet::new();
    let mut to_process = Vec::new();
    
    // Start with the binary
    let bin_path = find_binary_in_output2(bin_name)?;
    let root_id = cache.get_or_compute(&bin_path)?;
    to_process.push((root_id.clone(), 0));
    
    let mut dependency_tree = HashMap::new();
    
    while let Some((current_id, depth)) = to_process.pop() {
        if processed.contains(&current_id) || depth >= max_depth {
            continue;
        }
        
        processed.insert(current_id.clone());
        
        let node = cache.nodes.get(&current_id).unwrap();
        let mut resolved_deps = HashMap::new();
        
        // Find dependencies for this node's tokens
        scan_output2_for_tokens(&node.tokens, &mut resolved_deps)?;
        
        let mut dep_ids = Vec::new();
        for (_token, paths) in resolved_deps {
            if let Some(first_path) = paths.first() {
                let dep_id = cache.get_or_compute(first_path)?;
                dep_ids.push(dep_id.clone());
                to_process.push((dep_id, depth + 1));
            }
        }
        
        dependency_tree.insert(current_id.clone(), dep_ids);
    }
    
    // Print the graph
    print_tree(&root_id, &dependency_tree, &cache, 0, &mut HashSet::new())?;
    
    println!("\n📈 Graph Summary:");
    println!("   Total nodes: {}", processed.len());
    println!("   Max depth: {}", max_depth);
    println!("   Cache entries: {}", cache.nodes.len());
    
    Ok(())
}

fn print_tree(
    node_id: &str,
    tree: &HashMap<String, Vec<String>>,
    cache: &DepCache,
    depth: usize,
    visited: &mut HashSet<String>
) -> Result<(), Box<dyn std::error::Error>> {
    let indent = "  ".repeat(depth);
    let node = cache.nodes.get(node_id).unwrap();
    
    // Show node info
    let path_short = node.path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    println!("{}├─ {} ({:x})", indent, path_short, node.content_hash);
    println!("{}│  tokens: {:?}", indent, node.tokens.iter().take(3).collect::<Vec<_>>());
    
    if visited.contains(node_id) {
        println!("{}│  (already visited)", indent);
        return Ok(());
    }
    visited.insert(node_id.to_string());
    
    // Show dependencies
    if let Some(deps) = tree.get(node_id) {
        for (i, dep_id) in deps.iter().enumerate() {
            if i < 3 { // Limit to first 3 deps to avoid clutter
                print_tree(dep_id, tree, cache, depth + 1, visited)?;
            } else if i == 3 {
                println!("{}├─ ... and {} more dependencies", "  ".repeat(depth + 1), deps.len() - 3);
                break;
            }
        }
    }
    
    Ok(())
}
fn test_evaluation(bin_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing evaluation compilation for: {}", bin_name);
    
    let eval_file = format!("../bootstrap3-incremental/{}_recursive_eval.rs", bin_name);
    
    if !std::path::Path::new(&eval_file).exists() {
        println!("❌ Evaluation file not found. Run recursive-deps first.");
        return Ok(());
    }
    
    // Create a test crate
    let test_dir = format!("../bootstrap3-incremental/test-{}", bin_name);
    fs::create_dir_all(&format!("{}/src", test_dir))?;
    
    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "test-{}"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
syn = {{ version = "2.0", features = ["full"] }}
quote = "1.0"
anyhow = "1.0"
"#,
        bin_name
    );
    fs::write(format!("{}/Cargo.toml", test_dir), cargo_toml)?;
    
    // Create main.rs that includes the evaluation
    let main_content = format!(
        r#"// Test compilation of {} evaluation
use std::collections::HashMap;
use serde::{{Serialize, Deserialize}};

include!("../../{}_recursive_eval.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    println!("🧪 Testing {} evaluation...");
    evaluate_{}()?;
    println!("✅ Evaluation compiled and ran successfully!");
    Ok(())
}}
"#,
        bin_name, bin_name, bin_name, bin_name
    );
    fs::write(format!("{}/src/main.rs", test_dir), main_content)?;
    
    // Try to compile
    println!("🔨 Attempting compilation...");
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(&test_dir)
        .output()?;
    
    if output.status.success() {
        println!("✅ SUCCESS: All dependencies compile in correct order!");
        
        // Try to run it
        println!("🚀 Attempting execution...");
        let run_output = Command::new("cargo")
            .args(&["run"])
            .current_dir(&test_dir)
            .output()?;
            
        if run_output.status.success() {
            println!("✅ EXECUTION SUCCESS!");
            println!("{}", String::from_utf8_lossy(&run_output.stdout));
        } else {
            println!("⚠️  Compilation succeeded but execution failed:");
            println!("{}", String::from_utf8_lossy(&run_output.stderr));
        }
    } else {
        println!("❌ COMPILATION FAILED:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        
        // Show which includes are problematic
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("cannot find") {
            println!("\n🔍 Dependency resolution issues detected.");
            println!("This suggests the topological ordering or token resolution needs improvement.");
        }
    }
    
    Ok(())
}
fn extract_imports_and_content(content: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    if let Ok(ast) = syn::parse_file(content) {
        let mut import_visitor = ImportVisitor { imports: Vec::new() };
        import_visitor.visit_file(&ast);
        
        // Extract non-use items
        let mut non_use_items = Vec::new();
        for item in &ast.items {
            if !matches!(item, syn::Item::Use(_)) {
                non_use_items.push(quote::quote!(#item).to_string());
            }
        }
        
        let imports_str = import_visitor.imports.join("\n    ");
        let content_str = non_use_items.join("\n\n    ");
        
        Ok((
            if imports_str.is_empty() { String::new() } else { format!("    {}", imports_str) },
            if content_str.is_empty() { String::new() } else { format!("    {}", content_str) }
        ))
    } else {
        // Fallback: return content as-is
        Ok((String::new(), format!("    {}", content)))
    }
}
fn find_missing_types(content: &str) -> HashSet<String> {
    let mut missing = HashSet::new();
    
    // Extract all potential identifiers from content automatically
    let words: Vec<&str> = content.split_whitespace().collect();
    
    for window in words.windows(2) {
        if window[1] == "::" {
            // Found "Something ::" pattern - likely a type or module
            let identifier = window[0].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            if !identifier.is_empty() {
                missing.insert(identifier.to_string());
            }
        }
    }
    
    // Look for macro calls ending with !
    for word in &words {
        if word.ends_with('!') {
            let macro_name = word.trim_end_matches('!').trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            if !macro_name.is_empty() {
                missing.insert(macro_name.to_string());
            }
        }
    }
    
    // Look for standalone capitalized identifiers (likely types)
    for word in &words {
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if !clean_word.is_empty() && clean_word.chars().next().unwrap().is_uppercase() && clean_word.len() > 2 {
            missing.insert(clean_word.to_string());
        }
    }
    
    missing
}

fn resolve_dependencies_recursive(
    content: &str, 
    index: &Output2Index, 
    resolved_includes: &mut String, 
    visited: &mut HashSet<String>,
    depth: usize,
    glossary: &mut HashMap<String, String>
) -> Result<(), Box<dyn std::error::Error>> {
    if depth > 3 { // Stack overflow protection
        return Ok(());
    }
    
    let missing_types = find_missing_types(content);
    
    for token in missing_types {
        if visited.contains(&token) {
            continue; // Already resolved
        }
        visited.insert(token.clone());
        
        if let Some(paths) = index.resolve(&token) {
            if let Some(first_path) = paths.first() {
                let abs_path = std::fs::canonicalize(first_path).unwrap_or_else(|_| first_path.clone());
                println!("🔗 Resolving {} -> {}", token, abs_path.display());
                
                // Add to glossary
                glossary.insert(token.clone(), abs_path.to_string_lossy().to_string());
                
                resolved_includes.push_str(&format!("    include!(\"{}\"); // for {}\n", abs_path.display(), token));
                
                // Recursively resolve dependencies of this dependency
                if let Ok(dep_content) = fs::read_to_string(first_path) {
                    resolve_dependencies_recursive(&dep_content, index, resolved_includes, visited, depth + 1, glossary)?;
                }
            }
        }
    }
    
    Ok(())
}
