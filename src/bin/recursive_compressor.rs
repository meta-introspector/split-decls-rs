use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LayerVocab {
    layer: u8,
    vocab: HashMap<String, String>, // emoji -> pattern
    compression_ratio: f64,
    parent_layer: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecursiveCompression {
    layers: Vec<LayerVocab>,
    total_compression: f64,
    original_patterns: usize,
    final_vocab_size: usize,
}

fn compress_layer(input_vocab: &HashMap<String, String>, layer_num: u8) -> Result<LayerVocab> {
    let mut compressed = HashMap::new();
    let mut pattern_counts: HashMap<String, usize> = HashMap::new();
    
    // Count pattern fragments
    for pattern in input_vocab.values() {
        let fragments: Vec<&str> = pattern.split(&['(', ')', ',', ' ']).collect();
        for fragment in fragments {
            if !fragment.is_empty() && fragment.len() > 2 {
                *pattern_counts.entry(fragment.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Find most common fragments for this layer
    let mut sorted_fragments: Vec<_> = pattern_counts.into_iter().collect();
    sorted_fragments.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Create emoji vocab for top fragments
    let emojis = [
        "🔥", "⚡", "🎯", "🚀", "💎", "🌟", "🔮", "🎨", 
        "🎪", "🎭", "🦄", "🐉", "🦋", "🌸", "🍀", "🎲",
        "💫", "✨", "🌈", "🎨", "🎯", "🔥", "💎", "🌟"
    ];
    
    for (i, (fragment, count)) in sorted_fragments.iter().take(100).enumerate() {
        if count > &2 { // Only compress if appears more than twice
            let emoji_idx = (i + layer_num as usize * 100) % emojis.len();
            let emoji = emojis[emoji_idx];
            compressed.insert(emoji.to_string(), fragment.clone());
        }
    }
    
    let compression_ratio = compressed.len() as f64 / input_vocab.len() as f64;
    
    Ok(LayerVocab {
        layer: layer_num,
        vocab: compressed,
        compression_ratio,
        parent_layer: if layer_num > 0 { Some(layer_num - 1) } else { None },
    })
}

fn apply_compression(text: &str, vocab: &HashMap<String, String>) -> String {
    let mut result = text.to_string();
    
    // Sort by length (longest first) to avoid partial replacements
    let mut sorted_patterns: Vec<_> = vocab.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (emoji, pattern) in sorted_patterns {
        result = result.replace(pattern, emoji);
    }
    
    result
}

fn recursive_compress(initial_vocab: HashMap<String, String>) -> Result<RecursiveCompression> {
    let mut layers = Vec::new();
    let mut current_vocab = initial_vocab.clone();
    let original_size = initial_vocab.len();
    
    println!("🔄 RECURSIVE COMPRESSION - 8 Layers Deep");
    println!("========================================");
    
    for layer in 0..8 {
        println!("\n📊 Layer {}: Processing {} patterns", layer, current_vocab.len());
        
        let layer_vocab = compress_layer(&current_vocab, layer)?;
        println!("   Compressed to {} emoji tokens (ratio: {:.2})", 
            layer_vocab.vocab.len(), 
            layer_vocab.compression_ratio
        );
        
        // Apply compression to create next layer input
        let mut next_vocab = HashMap::new();
        for (emoji, pattern) in &current_vocab {
            let compressed_pattern = apply_compression(pattern, &layer_vocab.vocab);
            if compressed_pattern != *pattern { // Only keep if actually compressed
                next_vocab.insert(emoji.clone(), compressed_pattern);
            }
        }
        
        // Show sample compressions
        let samples: Vec<_> = layer_vocab.vocab.iter().take(5).collect();
        for (emoji, pattern) in samples {
            println!("   {} = {}", emoji, pattern);
        }
        
        layers.push(layer_vocab);
        current_vocab = next_vocab;
        
        if current_vocab.is_empty() {
            println!("   🎯 Compression complete at layer {}", layer);
            break;
        }
    }
    
    let final_size = layers.last().map(|l| l.vocab.len()).unwrap_or(0);
    let total_compression = final_size as f64 / original_size as f64;
    
    Ok(RecursiveCompression {
        layers,
        total_compression,
        original_patterns: original_size,
        final_vocab_size: final_size,
    })
}

fn main() -> Result<()> {
    println!("🎨 RECURSIVE EMOJI COMPRESSION SYSTEM");
    println!("====================================");
    
    // Load the original emoji mapping
    let emoji_map_content = fs::read_to_string("emoji_subexpression_map.json")?;
    let original_vocab: HashMap<String, String> = serde_json::from_str(&emoji_map_content)?;
    
    println!("📥 Loaded {} original patterns", original_vocab.len());
    
    // Perform recursive compression
    let compression_result = recursive_compress(original_vocab)?;
    
    // Display results
    println!("\n🎯 COMPRESSION SUMMARY:");
    println!("======================");
    println!("Original patterns: {}", compression_result.original_patterns);
    println!("Final vocab size: {}", compression_result.final_vocab_size);
    println!("Total compression ratio: {:.4}", compression_result.total_compression);
    println!("Compression layers: {}", compression_result.layers.len());
    
    // Show layer breakdown
    println!("\n📊 LAYER BREAKDOWN:");
    for layer in &compression_result.layers {
        println!("Layer {}: {} tokens (ratio: {:.3})", 
            layer.layer, 
            layer.vocab.len(), 
            layer.compression_ratio
        );
    }
    
    // Save compressed vocabulary
    let output = serde_json::to_string_pretty(&compression_result)?;
    fs::write("recursive_emoji_compression.json", output)?;
    println!("\n💾 Saved recursive compression to recursive_emoji_compression.json");
    
    // Create final ultra-compressed vocab
    let final_layer = compression_result.layers.last().unwrap();
    println!("\n🔥 FINAL ULTRA-COMPRESSED VOCABULARY:");
    println!("====================================");
    
    for (emoji, pattern) in final_layer.vocab.iter().take(20) {
        println!("{} = {}", emoji, pattern);
    }
    
    Ok(())
}
