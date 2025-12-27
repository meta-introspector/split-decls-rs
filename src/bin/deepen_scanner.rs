use anyhow::Result;
use crate::{
    rdf_url_blob::RdfUrlBlob,
    output2_macro_system::Output2MacroSystem,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🔍 Deepen Scanner - Self-Analysis Mode");
    
    // Load own source code as tape
    let mut tape_macros = HashMap::new();
    let self_code = fs::read_to_string("src/bin/deepen_scanner.rs")?;
    tape_macros.insert("deepen_scanner_self".to_string(), self_code);
    println!("📼 Loaded {} macros from tape (self-code)", tape_macros.len());
    
    // Scan output2 for similar blocks
    let output2_blocks = scan_output2_blocks()?;
    println!("📂 Found {} blocks in output2", output2_blocks.len());
    
    // Find similarities
    let similarities = find_similarities(&tape_macros, &output2_blocks)?;
    
    // Report results
    print_similarity_report(&similarities);
    
    Ok(())
}

fn load_tape_macros() -> Result<HashMap<String, String>> {
    let mut macros = HashMap::new();
    
    if Path::new("repl_state.rdf").exists() {
        let rdf_content = fs::read_to_string("repl_state.rdf")?;
        // Parse RDF for macro definitions
        for line in rdf_content.lines() {
            if line.contains("sys:macro") {
                // Extract macro name and content from RDF
                if let Some(name) = extract_macro_name(line) {
                    if let Some(content) = extract_macro_content(line) {
                        macros.insert(name, content);
                    }
                }
            }
        }
    }
    
    Ok(macros)
}

fn scan_output2_blocks() -> Result<HashMap<String, String>> {
    let mut blocks = HashMap::new();
    
    if Path::new("output2").exists() {
        for entry in fs::read_dir("output2")? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(entry.path())?;
                let filename = entry.file_name().to_string_lossy().to_string();
                blocks.insert(filename, content);
            }
        }
    }
    
    Ok(blocks)
}

fn find_similarities(tape: &HashMap<String, String>, blocks: &HashMap<String, String>) -> Result<Vec<Similarity>> {
    let mut similarities = Vec::new();
    
    for (tape_name, tape_content) in tape {
        for (block_name, block_content) in blocks {
            let similarity_score = calculate_similarity(tape_content, block_content);
            if similarity_score > 0.3 { // Lower threshold to 30%
                similarities.push(Similarity {
                    tape_macro: tape_name.clone(),
                    output2_block: block_name.clone(),
                    score: similarity_score,
                });
            }
        }
    }
    
    similarities.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    Ok(similarities)
}

fn calculate_similarity(content1: &str, content2: &str) -> f64 {
    // More sensitive similarity detection
    let tokens1: Vec<&str> = content1.split_whitespace().collect();
    let tokens2: Vec<&str> = content2.split_whitespace().collect();
    
    // Check for common Rust patterns
    let rust_patterns = ["HashMap", "Result", "anyhow", "fs::", "std::", "use", "fn", "struct", "impl"];
    let pattern_matches = rust_patterns.iter()
        .filter(|pattern| content1.contains(*pattern) && content2.contains(*pattern))
        .count();
    
    let common_tokens = tokens1.iter()
        .filter(|token| tokens2.contains(token))
        .count();
    
    let total_tokens = (tokens1.len() + tokens2.len()) as f64;
    let token_similarity = if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens };
    
    // Boost similarity for Rust pattern matches
    let pattern_boost = (pattern_matches as f64) * 0.1;
    
    (token_similarity + pattern_boost).min(1.0)
}

fn extract_macro_name(line: &str) -> Option<String> {
    // Extract macro name from RDF line
    if let Some(start) = line.find("sys:macro") {
        if let Some(name_start) = line[start..].find('"') {
            if let Some(name_end) = line[start + name_start + 1..].find('"') {
                return Some(line[start + name_start + 1..start + name_start + 1 + name_end].to_string());
            }
        }
    }
    None
}

fn extract_macro_content(line: &str) -> Option<String> {
    // Extract macro content from RDF line
    if let Some(content_start) = line.rfind('"') {
        if let Some(content_end) = line[..content_start].rfind('"') {
            return Some(line[content_end + 1..content_start].to_string());
        }
    }
    None
}

fn print_similarity_report(similarities: &[Similarity]) {
    println!("\n🎯 Similarity Analysis Results:");
    println!("═══════════════════════════════");
    
    if similarities.is_empty() {
        println!("✅ No significant similarities found");
        return;
    }
    
    for sim in similarities {
        println!("📊 {:.1}% similarity", sim.score * 100.0);
        println!("   📼 Tape: {}", sim.tape_macro);
        println!("   📂 Block: {}", sim.output2_block);
        println!();
    }
}

#[derive(Debug)]
struct Similarity {
    tape_macro: String,
    output2_block: String,
    score: f64,
}
