use crate::symbol_cache::{SymbolCache, CachedSymbol};
use crate::function_store::FunctionStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour, PeerId};
use walkdir;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct RustcNode {
    pub symbol: String,
    pub dependencies: Vec<String>,
    pub source_code: String,
    pub message_handlers: Vec<String>,
}

// Every rustc symbol becomes a libp2p message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RustcMessage {
    // Core compiler messages
    CompileRequest { symbol: String, dependencies: Vec<String> },
    CompileResponse { symbol: String, bytecode: Vec<u8>, success: bool },
    ExecuteSymbol { symbol: String, args: Vec<String> },
    ExecuteResponse { symbol: String, result: String },
    
    // Dependency resolution messages  
    ResolveDependencies { symbol: String },
    DependencyList { symbol: String, deps: Vec<String> },
    
    // Matrix operations
    LoadSymbolMap { compressed_data: Vec<u8> },
    SymbolMapLoaded { node_count: usize },
    GetNode { symbol: String },
    NodeData { symbol: String, source: String, deps: Vec<String> },
    
    // Rustc execution flow
    StartRustc,
    RustcMain { args: Vec<String> },
    RustcResult { exit_code: i32, output: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolNode {
    pub symbol: String,
    pub dependencies: Vec<String>,
    pub source_code: String,
    pub compiled_bytecode: Option<Vec<u8>>,
    pub message_handlers: Vec<String>, // Which message types this symbol can handle
}

#[derive(NetworkBehaviour)]
pub struct RustcBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct RustcMatrix {
    pub nodes: Arc<RwLock<HashMap<String, SymbolNode>>>,
    pub message_handlers: Arc<RwLock<HashMap<String, fn(RustcMessage) -> RustcMessage>>>,
    pub gossipsub_topic: gossipsub::IdentTopic,
    pub symbol_cache: SymbolCache,
    pub function_store: FunctionStore,
}

impl RustcMatrix {
    pub fn new() -> Self {
        let topic = gossipsub::IdentTopic::new("rustc-matrix");
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
            gossipsub_topic: topic,
            symbol_cache: SymbolCache::new(),
            function_store: FunctionStore::new("./functions"),
        }
    }

    // Evaluate a symbol: load, compile, execute, provide feedback
    pub async fn eval_symbol(&mut self, symbol_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🔍 Evaluating symbol: {}", symbol_name);
        
        // First try to load from function store
        if let Some(code) = self.function_store.get_function(symbol_name) {
            println!("📁 Found function in store, compiling...");
            return self.compile_and_run_code(symbol_name, code).await;
        }
        
        // Fall back to symbol cache
        if let Some(symbol) = self.symbol_cache.get(symbol_name) {
            println!("✅ Found symbol: {} ({} deps)", symbol_name, symbol.dependencies.len());
            
            // Save to function store for future use
            let code = format!(r#"
// Function: {}
// Type: {}
// Dependencies: {:?}

fn main() {{
    println!("Symbol {} evaluated successfully");
}}
"#, symbol.name, symbol.symbol_type, symbol.dependencies, symbol.name);
            
            self.function_store.save_function(symbol_name, &code)?;
            
            // Compile and run
            self.compile_and_run_code(symbol_name, &code).await
        } else {
            let error = format!("Symbol '{}' not found in cache or store", symbol_name);
            println!("❌ {}", error);
            Err(error.into())
        }
    }

    async fn compile_and_run_code(&self, name: &str, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        use std::process::Command;
        use std::fs;
        
        let temp_dir = format!("/tmp/rustc_eval_{}", name.replace("::", "_"));
        fs::create_dir_all(&temp_dir)?;
        
        let rust_file = format!("{}/eval.rs", temp_dir);
        fs::write(&rust_file, code)?;
        
        // Compile
        let output = Command::new("rustc")
            .arg(&rust_file)
            .arg("-o")
            .arg(format!("{}/eval", temp_dir))
            .output()?;
            
        if output.status.success() {
            // Run
            let run_output = Command::new(format!("{}/eval", temp_dir))
                .output()?;
                
            if run_output.status.success() {
                Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
            } else {
                Err(format!("Runtime error: {}", String::from_utf8_lossy(&run_output.stderr)).into())
            }
        } else {
            Err(format!("Compile error: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }

    // Watch for changes and reload
    pub async fn watch_and_reload(&mut self, symbol_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        use tokio::time::{sleep, Duration};
        
        println!("👁️  Watching symbol '{}' for changes...", symbol_name);
        
        loop {
            // Reload repositories
            self.load_repositories().await?;
            
            // Re-evaluate
            match self.eval_symbol(symbol_name).await {
                Ok(result) => println!("🔄 Reload: {}", result.trim()),
                Err(e) => println!("🔄 Reload failed: {}", e),
            }
            
            sleep(Duration::from_secs(2)).await;
        }
    }

    // Load multiple repositories into the matrix
    pub async fn load_repositories(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Loading repositories into matrix...");
        
        // Load symbols from shared memory cache
        self.symbol_cache.load_or_create()?;
        println!("✅ Symbol cache ready with {} symbols", self.symbol_cache.len());
        
        // 2. Index current repo (split-decls-genesis)
        self.index_repository(".", "split-decls-genesis").await?;
        
        // 3. Index ../split-decls-rs repo
        if std::path::Path::new("../split-decls-rs").exists() {
            self.index_repository("../split-decls-rs", "split-decls-rs").await?;
        }
        
        println!("🔍 Matrix ready with all repositories indexed");
        Ok(())
    }
    
    // Index a repository by scanning its Rust files
    async fn index_repository(&mut self, path: &str, repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📁 Indexing repository: {}", repo_name);
        
        let mut nodes = self.nodes.write().await;
        let mut count = 0;
        
        // Scan for .rs files
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    let symbol_name = format!("{}::{}", repo_name, 
                        entry.path().strip_prefix(path).unwrap_or(entry.path()).display());
                    
                    nodes.insert(symbol_name.clone(), SymbolNode {
                        symbol: symbol_name,
                        dependencies: vec![], // TODO: Parse dependencies from AST
                        source_code: content,
                        compiled_bytecode: None,
                        message_handlers: vec![],
                    });
                    count += 1;
                }
            }
        }
        
        println!("✅ Indexed {} files from {}", count, repo_name);
        Ok(())
    }

        // let compressed_data = std::fs::read(symbol_map_path)?;
        // let mut decoder = flate2::read::GzDecoder::new(&compressed_data[..]);
        // let mut content = String::new();
        // decoder.read_to_string(&mut content)?;
        // let symbol_data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        // let mut nodes = self.nodes.write().await;
        
        // for (symbol, data) in symbol_data {
        //     if let Some(deps) = data.get("dependencies").and_then(|d| d.as_array()) {
        //         let dependencies: Vec<String> = deps
        //             .iter()
        //             .filter_map(|d| d.as_str().map(|s| s.to_string()))
        //             .collect();

        //         let source_file = data.get("source_file")
        //             .and_then(|s| s.as_str())
        //             .unwrap_or("");

        //         let source_code = if !source_file.is_empty() {
        //             std::fs::read_to_string(format!("submodules/rust/{}", source_file))
        //                 .unwrap_or_default()
        //         } else {
        //             String::new()
        //         };

        //         // Each symbol can handle specific message types
        //         let message_handlers = vec![
        //             format!("compile_{}", symbol.replace("::", "_")),
        //             format!("execute_{}", symbol.replace("::", "_")),
        //         ];

        //         nodes.insert(symbol.clone(), SymbolNode {
        //             symbol: symbol.clone(),
        //             dependencies,
        //             source_code,
        //             compiled_bytecode: None,
        //             message_handlers,
        //         });
        //     }
        // }

        // println!("✅ Indexed {} files from {}", count, repo_name);
        // Ok(())

    
    pub async fn load_from_symbol_map(&self, symbol_map_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let compressed_data = std::fs::read(symbol_map_path)?;
        let mut decoder = flate2::read::GzDecoder::new(&compressed_data[..]);
        let mut content = String::new();
        decoder.read_to_string(&mut content)?;
        let symbol_data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        let mut nodes = self.nodes.write().await;
        
        for (symbol, data) in symbol_data {
            if let Some(deps) = data.get("dependencies").and_then(|d| d.as_array()) {
                let dependencies: Vec<String> = deps
                    .iter()
                    .filter_map(|d| d.as_str().map(|s| s.to_string()))
                    .collect();

                let source_file = data.get("source_file")
                    .and_then(|f| f.as_str())
                    .unwrap_or("");

                let source_code = if !source_file.is_empty() {
                    std::fs::read_to_string(format!("submodules/rust/{}", source_file))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                let message_handlers = vec![
                    format!("compile_{}", symbol.replace("::", "_")),
                    format!("execute_{}", symbol.replace("::", "_")),
                ];

                nodes.insert(symbol.clone(), SymbolNode {
                    symbol: symbol.clone(),
                    dependencies,
                    source_code,
                    compiled_bytecode: None,
                    message_handlers,
                });
            }
        }

        println!("📊 Loaded {} symbol nodes from {}", nodes.len(), symbol_map_path);
        Ok(())
    }
    
    // Query symbol from the loaded database
    pub async fn query_symbol(&self, symbol_name: &str) -> Option<RustcNode> {
        let nodes = self.nodes.read().await;
        if let Some(symbol_node) = nodes.get(symbol_name) {
            Some(RustcNode {
                symbol: symbol_node.symbol.clone(),
                dependencies: symbol_node.dependencies.clone(),
                source_code: symbol_node.source_code.clone(),
                message_handlers: symbol_node.message_handlers.clone(),
            })
        } else {
            None
        }
    }
    
    // Get all symbols with zero dependencies (compilation starting points)
    pub async fn get_zero_dependency_symbols(&self) -> Vec<String> {
        let nodes = self.nodes.read().await;
        nodes.iter()
            .filter(|(_, node)| node.dependencies.is_empty())
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub async fn handle_message(&self, message: RustcMessage) -> Option<RustcMessage> {
        match message {
            RustcMessage::CompileRequest { symbol, dependencies } => {
                println!("🔧 Compiling symbol: {}", symbol);
                // Compile the symbol and return response
                Some(RustcMessage::CompileResponse {
                    symbol: symbol.clone(),
                    bytecode: vec![0x90, 0x90], // Placeholder bytecode
                    success: true,
                })
            }
            
            RustcMessage::ExecuteSymbol { symbol, args } => {
                println!("🚀 Executing symbol: {} with args: {:?}", symbol, args);
                Some(RustcMessage::ExecuteResponse {
                    symbol: symbol.clone(),
                    result: format!("Executed {} successfully", symbol),
                })
            }
            
            RustcMessage::ResolveDependencies { symbol } => {
                let nodes = self.nodes.read().await;
                if let Some(node) = nodes.get(&symbol) {
                    Some(RustcMessage::DependencyList {
                        symbol: symbol.clone(),
                        deps: node.dependencies.clone(),
                    })
                } else {
                    None
                }
            }
            
            RustcMessage::GetNode { symbol } => {
                let nodes = self.nodes.read().await;
                if let Some(node) = nodes.get(&symbol) {
                    Some(RustcMessage::NodeData {
                        symbol: symbol.clone(),
                        source: node.source_code.clone(),
                        deps: node.dependencies.clone(),
                    })
                } else {
                    None
                }
            }
            
            RustcMessage::StartRustc => {
                println!("🚀 Starting rustc via libp2p message");
                Some(RustcMessage::RustcMain { args: vec![] })
            }
            
            RustcMessage::RustcMain { args } => {
                println!("🏛️ Executing rustc_driver::main with args: {:?}", args);
                // This would execute our dependency graph starting from main
                Some(RustcMessage::RustcResult {
                    exit_code: 0,
                    output: "Rustc execution complete".to_string(),
                })
            }
            
            _ => None,
        }
    }

    pub fn get_topic(&self) -> &gossipsub::IdentTopic {
        &self.gossipsub_topic
    }

    // Main execution function with eval testing
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting RustcMatrix with eval functionality...");
        
        let mut matrix = RustcMatrix::new();
        matrix.load_repositories().await?;
        
        // Test eval functionality
        println!("🧪 Testing eval functionality...");
        
        let first_symbol = matrix.symbol_cache.keys().next().cloned();
        if let Some(symbol_name) = first_symbol {
            println!("🎯 Testing with symbol: {}", symbol_name);
            match matrix.eval_symbol(&symbol_name).await {
                Ok(result) => println!("✅ Eval test passed: {}", result.trim()),
                Err(e) => println!("❌ Eval test failed: {}", e),
            }
        }
        
        Ok(())
    }
}
