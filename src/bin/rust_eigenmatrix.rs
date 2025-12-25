use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CachedMatrix {
    matrix: Vec<Vec<f64>>,
    decl_names: Vec<String>,
    timestamp: u64,
}

fn main() -> Result<()> {
    println!("🔥 PARALLEL Rust Diagonalization - 24 CPU EIGENMATRIX BEAST");
    
    // Load all declarations
    let decls = load_all_declarations()?;
    println!("📊 Loaded {} declarations", decls.len());
    
    // Try to load cached matrix
    let cache_file = "eigenmatrix_cache.json";
    if let Ok(cached) = load_cached_matrix(cache_file) {
        if cached.decl_names.len() == decls.len() {
            println!("⚡ Using cached matrix!");
            analyze_eigenmatrix(&cached.matrix, &decls);
            return Ok(());
        }
    }
    
    println!("🚀 Computing {}x{} eigenmatrix with PARALLEL POWER...", decls.len(), decls.len());
    
    // Create eigenmatrix with PARALLEL COMPUTATION
    let eigenmatrix = create_parallel_eigenmatrix(&decls)?;
    
    // Cache the results
    save_cached_matrix(cache_file, &eigenmatrix, &decls)?;
    
    // Analyze and report
    analyze_eigenmatrix(&eigenmatrix, &decls);
    
    Ok(())
}

fn create_parallel_eigenmatrix(decls: &[Declaration]) -> Result<Vec<Vec<f64>>> {
    let n = decls.len();
    let progress = Arc::new(Mutex::new(0));
    
    println!("🔥 FIRING UP ALL {} CORES!", rayon::current_num_threads());
    
    // Parallel computation of similarity matrix
    let matrix: Vec<Vec<f64>> = (0..n).into_par_iter().map(|i| {
        let row: Vec<f64> = (0..n).into_par_iter().map(|j| {
            if i == j {
                1.0 // Self-similarity
            } else {
                calculate_similarity(&decls[i].content, &decls[j].content)
            }
        }).collect();
        
        // Progress tracking
        {
            let mut p = progress.lock().unwrap();
            *p += 1;
            if *p % 10 == 0 {
                println!("🔥 CPU MELTING: {}/{} rows computed", *p, n);
            }
        }
        
        row
    }).collect();
    
    println!("💥 EIGENMATRIX COMPUTATION COMPLETE!");
    Ok(matrix)
}

fn calculate_similarity(content1: &str, content2: &str) -> f64 {
    // Optimized similarity calculation
    let tokens1: Vec<&str> = content1.split_whitespace().collect();
    let tokens2: Vec<&str> = content2.split_whitespace().collect();
    
    // Heavy Rust pattern matching for CPU load
    let rust_patterns = [
        "HashMap", "Result", "anyhow", "fs::", "std::", "use", "fn", "struct", "impl", 
        "Vec", "String", "Option", "match", "if", "let", "mut", "pub", "mod", "crate",
        "derive", "Clone", "Debug", "Default", "Serialize", "Deserialize", "Error"
    ];
    
    let pattern_matches = rust_patterns.par_iter()
        .filter(|pattern| content1.contains(*pattern) && content2.contains(*pattern))
        .count();
    
    let common_tokens = tokens1.par_iter()
        .filter(|token| tokens2.contains(token))
        .count();
    
    let total_tokens = (tokens1.len() + tokens2.len()) as f64;
    let token_similarity = if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens };
    
    // Pattern boost with more computation
    let pattern_boost = (pattern_matches as f64) * 0.03;
    let length_factor = (tokens1.len().min(tokens2.len()) as f64).sqrt() * 0.001;
    
    (token_similarity + pattern_boost + length_factor).min(1.0)
}

fn load_cached_matrix(cache_file: &str) -> Result<CachedMatrix> {
    let content = fs::read_to_string(cache_file)?;
    let cached: CachedMatrix = serde_json::from_str(&content)?;
    Ok(cached)
}

fn save_cached_matrix(cache_file: &str, matrix: &[Vec<f64>], decls: &[Declaration]) -> Result<()> {
    let cached = CachedMatrix {
        matrix: matrix.to_vec(),
        decl_names: decls.iter().map(|d| d.name.clone()).collect(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };
    
    let json = serde_json::to_string(&cached)?;
    fs::write(cache_file, json)?;
    println!("💾 Matrix cached to {}", cache_file);
    Ok(())
}

fn load_all_declarations() -> Result<Vec<Declaration>> {
    let mut decls = Vec::new();
    
    if Path::new("output2").exists() {
        load_declarations_recursive("output2", &mut decls)?;
    }
    
    Ok(decls)
}

fn load_declarations_recursive(dir: &str, decls: &mut Vec<Declaration>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            load_declarations_recursive(&path.to_string_lossy(), decls)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(&path)?;
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            decls.push(Declaration { name, content, path: path.to_string_lossy().to_string() });
        }
    }
    
    Ok(())
}

fn analyze_eigenmatrix(matrix: &[Vec<f64>], decls: &[Declaration]) {
    println!("\n🎯 PARALLEL Eigenmatrix Analysis Results:");
    println!("═══════════════════════════════════════");
    
    // Parallel similarity computation
    let mut similarities: Vec<(f64, usize, usize)> = (0..matrix.len()).into_par_iter()
        .flat_map(|i| {
            ((i+1)..matrix[i].len()).into_par_iter()
                .map(move |j| (matrix[i][j], i, j))
        })
        .collect();
    
    similarities.par_sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    println!("🔝 Top 15 Similar Declaration Pairs:");
    for (score, i, j) in similarities.iter().take(15) {
        println!("   {:.1}% - {} ↔ {}", score * 100.0, 
                 decls[*i].name.chars().take(35).collect::<String>(),
                 decls[*j].name.chars().take(35).collect::<String>());
    }
    
    // Parallel eigenvalue approximation
    let eigenvals: Vec<(f64, usize)> = matrix.par_iter().enumerate()
        .map(|(i, row)| (row.par_iter().sum::<f64>(), i))
        .collect();
    
    let mut eigenvals = eigenvals;
    eigenvals.par_sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    println!("\n🧮 Top Eigenvalue Approximations:");
    for (eigenval, i) in eigenvals.iter().take(8) {
        println!("   λ={:.2} - {}", eigenval, 
                 decls[*i].name.chars().take(45).collect::<String>());
    }
    
    // Parallel statistics
    let avg_similarity: f64 = similarities.par_iter().map(|(s, _, _)| s).sum::<f64>() / similarities.len() as f64;
    let high_similarity_count = similarities.par_iter().filter(|(s, _, _)| *s > 0.5).count();
    
    println!("\n📈 BEAST MODE Matrix Statistics:");
    println!("   🔥 Dimensions: {}x{}", matrix.len(), matrix[0].len());
    println!("   🔥 Total comparisons: {}", similarities.len());
    println!("   🔥 Average similarity: {:.1}%", avg_similarity * 100.0);
    println!("   🔥 Max similarity: {:.1}%", similarities[0].0 * 100.0);
    println!("   🔥 High similarity pairs (>50%): {}", high_similarity_count);
    println!("   🔥 CPU cores utilized: {}", rayon::current_num_threads());
}

#[derive(Debug)]
struct Declaration {
    name: String,
    content: String,
    path: String,
}
