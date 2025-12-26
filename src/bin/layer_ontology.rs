use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LayerPattern {
    pattern: String,
    frequency: usize,
    nodes: Vec<String>,
    emoji: String,
    semantic_label: String,
    layer: CompilerLayer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum CompilerLayer {
    AST,      // Abstract Syntax Tree
    HIR,      // High-level IR
    MIR,      // Mid-level IR
    THIR,     // Typed HIR
    Parser,   // Parsing layer
    Resolve,  // Name resolution
    TypeCk,   // Type checking
    Codegen,  // Code generation
    Lint,     // Linting
    Macro,    // Macro expansion
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct LayerOntology {
    layer: CompilerLayer,
    ngram_size: usize,
    top_patterns: Vec<LayerPattern>,
    total_patterns: usize,
    coverage_percentage: f64,
    cross_layer_relations: Vec<CrossLayerRelation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrossLayerRelation {
    pattern: String,
    source_layer: CompilerLayer,
    target_layer: CompilerLayer,
    relation_strength: f64,
    shared_nodes: usize,
}

struct LayerAnalyzer {
    patterns: HashMap<(CompilerLayer, String), (usize, Vec<String>)>,
    emoji_map: HashMap<String, String>,
}

impl CompilerLayer {
    fn from_path(path: &Path) -> Self {
        let path_str = path.to_string_lossy().to_lowercase();
        
        if path_str.contains("ast") || path_str.contains("syntax") {
            CompilerLayer::AST
        } else if path_str.contains("hir") && !path_str.contains("thir") {
            CompilerLayer::HIR
        } else if path_str.contains("mir") {
            CompilerLayer::MIR
        } else if path_str.contains("thir") {
            CompilerLayer::THIR
        } else if path_str.contains("parse") || path_str.contains("lexer") {
            CompilerLayer::Parser
        } else if path_str.contains("resolve") || path_str.contains("name") {
            CompilerLayer::Resolve
        } else if path_str.contains("typeck") || path_str.contains("type") {
            CompilerLayer::TypeCk
        } else if path_str.contains("codegen") || path_str.contains("llvm") {
            CompilerLayer::Codegen
        } else if path_str.contains("lint") {
            CompilerLayer::Lint
        } else if path_str.contains("macro") || path_str.contains("expand") {
            CompilerLayer::Macro
        } else {
            CompilerLayer::Other(
                path.parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            )
        }
    }
    
    fn emoji(&self) -> &'static str {
        match self {
            CompilerLayer::AST => "🌲",
            CompilerLayer::HIR => "🌳", 
            CompilerLayer::MIR => "🔬",
            CompilerLayer::THIR => "🏷️",
            CompilerLayer::Parser => "📖",
            CompilerLayer::Resolve => "🔍",
            CompilerLayer::TypeCk => "✅",
            CompilerLayer::Codegen => "⚙️",
            CompilerLayer::Lint => "🔍",
            CompilerLayer::Macro => "🎭",
            CompilerLayer::Other(_) => "📁",
        }
    }
}

impl LayerAnalyzer {
    fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            emoji_map: Self::load_layer_emoji_map(),
        }
    }
    
    fn load_layer_emoji_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        
        // AST patterns
        map.insert("ast".to_string(), "🌲".to_string());
        map.insert("node".to_string(), "🔵".to_string());
        map.insert("expr".to_string(), "💭".to_string());
        map.insert("stmt".to_string(), "📝".to_string());
        map.insert("item".to_string(), "📄".to_string());
        map.insert("pat".to_string(), "🎨".to_string());
        map.insert("ty".to_string(), "🏷️".to_string());
        
        // HIR patterns
        map.insert("hir".to_string(), "🌳".to_string());
        map.insert("def".to_string(), "📋".to_string());
        map.insert("body".to_string(), "🫀".to_string());
        
        // MIR patterns
        map.insert("mir".to_string(), "🔬".to_string());
        map.insert("basic".to_string(), "🧱".to_string());
        map.insert("block".to_string(), "🧱".to_string());
        map.insert("terminator".to_string(), "🔚".to_string());
        
        // Parser patterns
        map.insert("parse".to_string(), "📖".to_string());
        map.insert("token".to_string(), "🎫".to_string());
        map.insert("lexer".to_string(), "🔤".to_string());
        
        // Type checking
        map.insert("check".to_string(), "✅".to_string());
        map.insert("infer".to_string(), "🤔".to_string());
        map.insert("trait".to_string(), "🔗".to_string());
        
        map
    }
    
    fn extract_layer_ngrams(&mut self, text: &str, n: usize, node_name: &str, layer: CompilerLayer) {
        let tokens: Vec<&str> = text.split_whitespace().collect();
        
        for window in tokens.windows(n) {
            let pattern = window.join(" ");
            let key = (layer.clone(), pattern);
            let entry = self.patterns.entry(key).or_insert((0, Vec::new()));
            entry.0 += 1;
            if !entry.1.contains(&node_name.to_string()) {
                entry.1.push(node_name.to_string());
            }
        }
    }
    
    fn analyze_file(&mut self, file_path: &Path, n: usize) -> Result<()> {
        if !file_path.extension().map_or(false, |ext| ext == "rs") {
            return Ok(());
        }
        
        let layer = CompilerLayer::from_path(file_path);
        let content = fs::read_to_string(file_path)?;
        let node_name = file_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Clean content for analysis
        let clean_content = content
            .lines()
            .filter(|line| !line.trim().starts_with("//") && !line.trim().starts_with("/*"))
            .collect::<Vec<_>>()
            .join(" ")
            .replace(&['{', '}', '(', ')', '[', ']', ';', ',', ':', '"'], " ");
        
        self.extract_layer_ngrams(&clean_content, n, &node_name, layer);
        Ok(())
    }
    
    fn get_layer_patterns(&self, layer: &CompilerLayer, top_k: usize) -> Vec<LayerPattern> {
        let mut layer_patterns: Vec<_> = self.patterns.iter()
            .filter(|((l, _), _)| l == layer)
            .collect();
        
        layer_patterns.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        
        layer_patterns.into_iter()
            .take(top_k)
            .map(|((layer, pattern), (freq, nodes))| {
                let emoji = self.assign_layer_emoji(pattern, layer);
                let semantic_label = format!("{}_{}", 
                    match layer {
                        CompilerLayer::AST => "ast",
                        CompilerLayer::HIR => "hir", 
                        CompilerLayer::MIR => "mir",
                        CompilerLayer::THIR => "thir",
                        CompilerLayer::Parser => "parser",
                        CompilerLayer::Resolve => "resolve",
                        CompilerLayer::TypeCk => "typeck",
                        CompilerLayer::Codegen => "codegen",
                        CompilerLayer::Lint => "lint",
                        CompilerLayer::Macro => "macro",
                        CompilerLayer::Other(s) => s,
                    },
                    pattern.replace(" ", "_")
                );
                
                LayerPattern {
                    pattern: pattern.clone(),
                    frequency: *freq,
                    nodes: nodes.clone(),
                    emoji,
                    semantic_label,
                    layer: layer.clone(),
                }
            })
            .collect()
    }
    
    fn assign_layer_emoji(&self, pattern: &str, layer: &CompilerLayer) -> String {
        // First try layer-specific emoji
        let layer_emoji = layer.emoji().to_string();
        
        // Then try pattern-specific emoji
        let words: Vec<&str> = pattern.split_whitespace().collect();
        for word in &words {
            if let Some(emoji) = self.emoji_map.get(*word) {
                return format!("{}{}", layer_emoji, emoji);
            }
        }
        
        layer_emoji
    }
    
    fn find_cross_layer_relations(&self, source_layer: &CompilerLayer, target_layer: &CompilerLayer) -> Vec<CrossLayerRelation> {
        let mut relations = Vec::new();
        
        let source_patterns: HashMap<String, &Vec<String>> = self.patterns.iter()
            .filter(|((l, _), _)| l == source_layer)
            .map(|((_, p), (_, nodes))| (p.clone(), nodes))
            .collect();
        
        let target_patterns: HashMap<String, &Vec<String>> = self.patterns.iter()
            .filter(|((l, _), _)| l == target_layer)
            .map(|((_, p), (_, nodes))| (p.clone(), nodes))
            .collect();
        
        for (pattern, source_nodes) in &source_patterns {
            if let Some(target_nodes) = target_patterns.get(pattern) {
                let shared_nodes = source_nodes.iter()
                    .filter(|node| target_nodes.contains(node))
                    .count();
                
                if shared_nodes > 0 {
                    let relation_strength = shared_nodes as f64 / 
                        (source_nodes.len().max(target_nodes.len()) as f64);
                    
                    relations.push(CrossLayerRelation {
                        pattern: pattern.clone(),
                        source_layer: source_layer.clone(),
                        target_layer: target_layer.clone(),
                        relation_strength,
                        shared_nodes,
                    });
                }
            }
        }
        
        relations.sort_by(|a, b| b.relation_strength.partial_cmp(&a.relation_strength).unwrap());
        relations
    }
}

fn analyze_compiler_layers(source_dir: &Path, n: usize, top_k: usize) -> Result<Vec<LayerOntology>> {
    let mut analyzer = LayerAnalyzer::new();
    let mut file_count = 0;
    
    println!("🦀 Analyzing compiler layers with {}-grams...", n);
    
    // Walk through all Rust files
    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Err(e) = analyzer.analyze_file(entry.path(), n) {
                eprintln!("Warning: Failed to analyze {:?}: {}", entry.path(), e);
            } else {
                file_count += 1;
                if file_count % 1000 == 0 {
                    println!("  Processed {} files...", file_count);
                }
            }
        }
    }
    
    let layers = vec![
        CompilerLayer::AST,
        CompilerLayer::HIR,
        CompilerLayer::MIR,
        CompilerLayer::THIR,
        CompilerLayer::Parser,
        CompilerLayer::Resolve,
        CompilerLayer::TypeCk,
        CompilerLayer::Codegen,
        CompilerLayer::Lint,
        CompilerLayer::Macro,
    ];
    
    let mut ontologies = Vec::new();
    
    for layer in &layers {
        let patterns = analyzer.get_layer_patterns(layer, top_k);
        let total_patterns = analyzer.patterns.iter()
            .filter(|((l, _), _)| l == layer)
            .count();
        
        if !patterns.is_empty() {
            // Find cross-layer relations
            let mut cross_relations = Vec::new();
            for other_layer in &layers {
                if other_layer != layer {
                    let relations = analyzer.find_cross_layer_relations(layer, other_layer);
                    cross_relations.extend(relations.into_iter().take(3)); // Top 3 relations per layer pair
                }
            }
            
            let coverage = if total_patterns > 0 {
                (patterns.iter().map(|p| p.frequency).sum::<usize>() as f64 / 
                 analyzer.patterns.iter()
                    .filter(|((l, _), (f, _))| l == layer)
                    .map(|(_, (f, _))| f)
                    .sum::<usize>() as f64) * 100.0
            } else {
                0.0
            };
            
            ontologies.push(LayerOntology {
                layer: layer.clone(),
                ngram_size: n,
                top_patterns: patterns,
                total_patterns,
                coverage_percentage: coverage,
                cross_layer_relations: cross_relations,
            });
        }
    }
    
    println!("✅ Analyzed {} files across {} compiler layers", file_count, ontologies.len());
    Ok(ontologies)
}

fn main() -> Result<()> {
    let source_dir = Path::new(".");
    let ngram_sizes = vec![2, 3, 5];
    let top_k = 5;
    
    println!("🦀 Compiler Layer N-Gram Ontology Generator");
    println!("============================================");
    
    for &n in &ngram_sizes {
        println!("\n📊 Analyzing {}-grams across compiler layers:", n);
        
        match analyze_compiler_layers(source_dir, n, top_k) {
            Ok(ontologies) => {
                for ontology in &ontologies {
                    println!("\n{} {} Layer Analysis:", ontology.layer.emoji(), 
                            match &ontology.layer {
                                CompilerLayer::AST => "AST",
                                CompilerLayer::HIR => "HIR",
                                CompilerLayer::MIR => "MIR", 
                                CompilerLayer::THIR => "THIR",
                                CompilerLayer::Parser => "Parser",
                                CompilerLayer::Resolve => "Resolve",
                                CompilerLayer::TypeCk => "TypeCheck",
                                CompilerLayer::Codegen => "Codegen",
                                CompilerLayer::Lint => "Lint",
                                CompilerLayer::Macro => "Macro",
                                CompilerLayer::Other(s) => s,
                            });
                    
                    println!("   Total patterns: {}", ontology.total_patterns);
                    println!("   Coverage: {:.2}%", ontology.coverage_percentage);
                    
                    for (i, pattern) in ontology.top_patterns.iter().take(3).enumerate() {
                        println!("   {}. {} \"{}\" (freq: {}, nodes: {})", 
                                i + 1, 
                                pattern.emoji,
                                pattern.pattern, 
                                pattern.frequency,
                                pattern.nodes.len());
                    }
                    
                    if !ontology.cross_layer_relations.is_empty() {
                        println!("   Cross-layer relations:");
                        for relation in ontology.cross_layer_relations.iter().take(2) {
                            println!("     → {} \"{}\" (strength: {:.2}, shared: {})",
                                    relation.target_layer.emoji(),
                                    relation.pattern,
                                    relation.relation_strength,
                                    relation.shared_nodes);
                        }
                    }
                }
                
                let filename = format!("compiler_layer_ontology_{}gram.json", n);
                let json = serde_json::to_string_pretty(&ontologies)?;
                fs::write(&filename, json)?;
                println!("\n💾 Saved to {}", filename);
            }
            Err(e) => {
                eprintln!("❌ Error analyzing {}-grams: {}", n, e);
            }
        }
    }
    
    println!("\n🎯 Compiler Layer Ontology Generation Complete!");
    Ok(())
}
