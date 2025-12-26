use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NGramPattern {
    pattern: String,
    frequency: usize,
    nodes: Vec<String>,
    emoji: String,
    semantic_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RustcOntology {
    ngram_size: usize,
    top_patterns: Vec<NGramPattern>,
    total_patterns: usize,
    coverage_percentage: f64,
}

struct NGramAnalyzer {
    patterns: HashMap<String, (usize, Vec<String>)>, // pattern -> (frequency, nodes)
    emoji_map: HashMap<String, String>,
}

impl NGramAnalyzer {
    fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            emoji_map: Self::load_emoji_map(),
        }
    }
    
    fn load_emoji_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        
        // Core Rust patterns
        map.insert("fn".to_string(), "🔧".to_string());
        map.insert("struct".to_string(), "📦".to_string());
        map.insert("impl".to_string(), "⚙️".to_string());
        map.insert("enum".to_string(), "🎯".to_string());
        map.insert("trait".to_string(), "🔗".to_string());
        map.insert("mod".to_string(), "📁".to_string());
        map.insert("use".to_string(), "📥".to_string());
        map.insert("pub".to_string(), "🌐".to_string());
        map.insert("let".to_string(), "📝".to_string());
        map.insert("match".to_string(), "🎲".to_string());
        map.insert("if".to_string(), "❓".to_string());
        map.insert("for".to_string(), "🔄".to_string());
        map.insert("while".to_string(), "⏳".to_string());
        map.insert("return".to_string(), "↩️".to_string());
        map.insert("mut".to_string(), "🔄".to_string());
        map.insert("const".to_string(), "🔒".to_string());
        map.insert("static".to_string(), "🏛️".to_string());
        map.insert("async".to_string(), "⚡".to_string());
        map.insert("await".to_string(), "⏰".to_string());
        map.insert("unsafe".to_string(), "⚠️".to_string());
        
        // Compiler-specific patterns
        map.insert("rustc".to_string(), "🦀".to_string());
        map.insert("hir".to_string(), "🌳".to_string());
        map.insert("mir".to_string(), "🔬".to_string());
        map.insert("ast".to_string(), "🌲".to_string());
        map.insert("ty".to_string(), "🏷️".to_string());
        map.insert("span".to_string(), "📍".to_string());
        map.insert("def".to_string(), "📋".to_string());
        map.insert("node".to_string(), "🔵".to_string());
        map.insert("item".to_string(), "📄".to_string());
        map.insert("expr".to_string(), "💭".to_string());
        map.insert("stmt".to_string(), "📝".to_string());
        map.insert("pat".to_string(), "🎨".to_string());
        map.insert("path".to_string(), "🛤️".to_string());
        map.insert("ident".to_string(), "🏷️".to_string());
        map.insert("token".to_string(), "🎫".to_string());
        map.insert("macro".to_string(), "🎭".to_string());
        map.insert("attr".to_string(), "🏷️".to_string());
        map.insert("vis".to_string(), "👁️".to_string());
        map.insert("gen".to_string(), "🧬".to_string());
        map.insert("param".to_string(), "⚙️".to_string());
        
        map
    }
    
    fn extract_ngrams(&mut self, text: &str, n: usize, node_name: &str) {
        let tokens: Vec<&str> = text.split_whitespace().collect();
        
        for window in tokens.windows(n) {
            let pattern = window.join(" ");
            let entry = self.patterns.entry(pattern).or_insert((0, Vec::new()));
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
        
        self.extract_ngrams(&clean_content, n, &node_name);
        Ok(())
    }
    
    fn get_top_patterns(&self, top_k: usize) -> Vec<NGramPattern> {
        let mut patterns: Vec<_> = self.patterns.iter().collect();
        patterns.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        
        patterns.into_iter()
            .take(top_k)
            .map(|(pattern, (freq, nodes))| {
                let emoji = self.assign_emoji(pattern);
                let semantic_label = self.generate_semantic_label(pattern);
                
                NGramPattern {
                    pattern: pattern.clone(),
                    frequency: *freq,
                    nodes: nodes.clone(),
                    emoji,
                    semantic_label,
                }
            })
            .collect()
    }
    
    fn assign_emoji(&self, pattern: &str) -> String {
        let words: Vec<&str> = pattern.split_whitespace().collect();
        
        // Try to find emoji for any word in the pattern
        for word in &words {
            if let Some(emoji) = self.emoji_map.get(*word) {
                return emoji.clone();
            }
        }
        
        // Fallback based on pattern characteristics
        if pattern.contains("error") || pattern.contains("panic") {
            "❌".to_string()
        } else if pattern.contains("test") {
            "🧪".to_string()
        } else if pattern.contains("debug") {
            "🐛".to_string()
        } else if pattern.contains("parse") {
            "📖".to_string()
        } else if pattern.contains("compile") {
            "⚙️".to_string()
        } else if pattern.contains("type") {
            "🏷️".to_string()
        } else if pattern.contains("check") {
            "✅".to_string()
        } else if pattern.contains("resolve") {
            "🔍".to_string()
        } else if pattern.contains("expand") {
            "📈".to_string()
        } else if pattern.contains("lower") {
            "⬇️".to_string()
        } else {
            "🔹".to_string()
        }
    }
    
    fn generate_semantic_label(&self, pattern: &str) -> String {
        let words: Vec<&str> = pattern.split_whitespace().collect();
        
        if words.len() == 1 {
            format!("single_token_{}", words[0])
        } else if words.len() <= 3 {
            format!("short_pattern_{}", words.join("_"))
        } else if words.len() <= 7 {
            format!("medium_pattern_{}", words[0])
        } else {
            format!("long_pattern_{}", words[0])
        }
    }
}

fn analyze_ngrams_for_size(source_dir: &Path, n: usize, top_k: usize) -> Result<RustcOntology> {
    let mut analyzer = NGramAnalyzer::new();
    let mut file_count = 0;
    
    println!("🔍 Analyzing {}-grams in {:?}...", n, source_dir);
    
    // Walk through all Rust files
    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Err(e) = analyzer.analyze_file(entry.path(), n) {
                eprintln!("Warning: Failed to analyze {:?}: {}", entry.path(), e);
            } else {
                file_count += 1;
                if file_count % 100 == 0 {
                    println!("  Processed {} files...", file_count);
                }
            }
        }
    }
    
    let total_patterns = analyzer.patterns.len();
    let top_patterns = analyzer.get_top_patterns(top_k);
    let coverage = if total_patterns > 0 {
        (top_patterns.iter().map(|p| p.frequency).sum::<usize>() as f64 / 
         analyzer.patterns.values().map(|(f, _)| f).sum::<usize>() as f64) * 100.0
    } else {
        0.0
    };
    
    println!("✅ Found {} unique {}-grams from {} files", total_patterns, n, file_count);
    
    Ok(RustcOntology {
        ngram_size: n,
        top_patterns,
        total_patterns,
        coverage_percentage: coverage,
    })
}

fn main() -> Result<()> {
    let source_dir = Path::new(".");
    let ngram_sizes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let top_counts = vec![2, 3, 5];
    
    println!("🦀 Rustc N-Gram Ontology Generator");
    println!("==================================");
    
    for &n in &ngram_sizes {
        for &top_k in &top_counts {
            println!("\n📊 Analyzing {}-grams (top {}):", n, top_k);
            
            match analyze_ngrams_for_size(source_dir, n, top_k) {
                Ok(ontology) => {
                    let filename = format!("rustc_ontology_{}gram_top{}.json", n, top_k);
                    let json = serde_json::to_string_pretty(&ontology)?;
                    fs::write(&filename, json)?;
                    
                    println!("📈 Results for {}-grams (top {}):", n, top_k);
                    println!("   Total patterns: {}", ontology.total_patterns);
                    println!("   Coverage: {:.2}%", ontology.coverage_percentage);
                    
                    for (i, pattern) in ontology.top_patterns.iter().enumerate() {
                        println!("   {}. {} {} \"{}\" (freq: {}, nodes: {})", 
                                i + 1, 
                                pattern.emoji, 
                                pattern.semantic_label,
                                pattern.pattern, 
                                pattern.frequency,
                                pattern.nodes.len());
                    }
                    
                    println!("💾 Saved to {}", filename);
                }
                Err(e) => {
                    eprintln!("❌ Error analyzing {}-grams: {}", n, e);
                }
            }
        }
    }
    
    println!("\n🎯 Core Rustc Ontology Generation Complete!");
    Ok(())
}
