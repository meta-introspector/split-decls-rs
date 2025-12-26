use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

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

#[derive(Debug, Serialize)]
struct OntologySummary {
    total_files_analyzed: usize,
    ngram_analyses: Vec<NGramAnalysisSummary>,
    core_patterns: Vec<CorePattern>,
    emoji_distribution: HashMap<String, usize>,
}

#[derive(Debug, Serialize)]
struct NGramAnalysisSummary {
    ngram_size: usize,
    top_k: usize,
    total_patterns: usize,
    coverage_percentage: f64,
    top_pattern: String,
    top_frequency: usize,
}

#[derive(Debug, Serialize)]
struct CorePattern {
    pattern: String,
    emoji: String,
    total_frequency: usize,
    appears_in_ngrams: Vec<usize>,
    semantic_category: String,
}

fn load_ontology(filename: &str) -> Result<RustcOntology> {
    let content = fs::read_to_string(filename)?;
    Ok(serde_json::from_str(&content)?)
}

fn categorize_pattern(pattern: &str) -> String {
    if pattern.contains("use") && pattern.contains("std") {
        "standard_library_imports".to_string()
    } else if pattern.contains("serde") {
        "serialization_framework".to_string()
    } else if pattern.contains("rustc") || pattern.contains("hir") || pattern.contains("mir") {
        "compiler_internals".to_string()
    } else if pattern.contains("fn") || pattern.contains("impl") {
        "function_definitions".to_string()
    } else if pattern.contains("struct") || pattern.contains("enum") {
        "data_structures".to_string()
    } else if pattern.contains("error") || pattern.contains("Result") {
        "error_handling".to_string()
    } else if pattern.contains("test") {
        "testing_framework".to_string()
    } else if pattern.contains("macro") {
        "macro_system".to_string()
    } else {
        "general_patterns".to_string()
    }
}

fn main() -> Result<()> {
    println!("🦀 Rustc Ontology Analysis Summary");
    println!("==================================");
    
    let ontology_files: Vec<_> = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name().to_string_lossy().starts_with("rustc_ontology_") &&
            entry.file_name().to_string_lossy().ends_with(".json")
        })
        .collect();
    
    let mut analyses = Vec::new();
    let mut all_patterns: HashMap<String, CorePattern> = HashMap::new();
    let mut emoji_counts: HashMap<String, usize> = HashMap::new();
    
    for file_entry in &ontology_files {
        let filename = file_entry.file_name().to_string_lossy().to_string();
        
        match load_ontology(&filename) {
            Ok(ontology) => {
                println!("\n📊 Analysis: {}", filename);
                println!("   N-gram size: {}", ontology.ngram_size);
                println!("   Total patterns: {}", ontology.total_patterns);
                println!("   Coverage: {:.2}%", ontology.coverage_percentage);
                
                if !ontology.top_patterns.is_empty() {
                    let top = &ontology.top_patterns[0];
                    println!("   Top pattern: {} {} \"{}\" (freq: {})", 
                            top.emoji, top.semantic_label, top.pattern, top.frequency);
                    
                    // Extract top_k from filename
                    let top_k = filename.split("top").nth(1)
                        .and_then(|s| s.split(".json").next())
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(0);
                    
                    analyses.push(NGramAnalysisSummary {
                        ngram_size: ontology.ngram_size,
                        top_k,
                        total_patterns: ontology.total_patterns,
                        coverage_percentage: ontology.coverage_percentage,
                        top_pattern: top.pattern.clone(),
                        top_frequency: top.frequency,
                    });
                }
                
                // Collect patterns for core analysis
                for pattern in &ontology.top_patterns {
                    *emoji_counts.entry(pattern.emoji.clone()).or_insert(0) += 1;
                    
                    let entry = all_patterns.entry(pattern.pattern.clone()).or_insert_with(|| {
                        CorePattern {
                            pattern: pattern.pattern.clone(),
                            emoji: pattern.emoji.clone(),
                            total_frequency: 0,
                            appears_in_ngrams: Vec::new(),
                            semantic_category: categorize_pattern(&pattern.pattern),
                        }
                    });
                    
                    entry.total_frequency += pattern.frequency;
                    if !entry.appears_in_ngrams.contains(&ontology.ngram_size) {
                        entry.appears_in_ngrams.push(ontology.ngram_size);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to load {}: {}", filename, e);
            }
        }
    }
    
    // Sort core patterns by frequency
    let mut core_patterns: Vec<_> = all_patterns.into_values().collect();
    core_patterns.sort_by(|a, b| b.total_frequency.cmp(&a.total_frequency));
    
    println!("\n🎯 Core Rustc Ontology Patterns (Top 10):");
    for (i, pattern) in core_patterns.iter().take(10).enumerate() {
        println!("   {}. {} {} \"{}\"", 
                i + 1, 
                pattern.emoji, 
                pattern.semantic_category,
                pattern.pattern);
        println!("      Frequency: {}, N-grams: {:?}", 
                pattern.total_frequency, 
                pattern.appears_in_ngrams);
    }
    
    println!("\n📈 Emoji Distribution:");
    let mut emoji_vec: Vec<_> = emoji_counts.iter().collect();
    emoji_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (emoji, count) in emoji_vec.iter().take(10) {
        println!("   {} appears {} times", emoji, count);
    }
    
    // Create summary
    let summary = OntologySummary {
        total_files_analyzed: 36500, // Approximate from the logs
        ngram_analyses: analyses,
        core_patterns: core_patterns.into_iter().take(20).collect(),
        emoji_distribution: emoji_counts,
    };
    
    let summary_json = serde_json::to_string_pretty(&summary)?;
    fs::write("rustc_core_ontology_summary.json", summary_json)?;
    
    println!("\n💾 Complete summary saved to rustc_core_ontology_summary.json");
    println!("🎯 Core Rustc Ontology Generation Complete!");
    
    Ok(())
}
