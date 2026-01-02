use std::io::Read;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour, PeerId};
use tokio::sync::RwLock;
use std::sync::Arc;

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
}

impl RustcMatrix {
    pub fn new() -> Self {
        let topic = gossipsub::IdentTopic::new("rustc-matrix");
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
            gossipsub_topic: topic,
        }
    }

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
                    .and_then(|s| s.as_str())
                    .unwrap_or("");

                let source_code = if !source_file.is_empty() {
                    std::fs::read_to_string(format!("submodules/rust/{}", source_file))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                // Each symbol can handle specific message types
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

        println!("📊 Loaded {} symbol nodes as message handlers", nodes.len());
        Ok(())
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
}
