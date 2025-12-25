use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🔬 Rust Diagonalization - Eigenmatrix Analysis");
    
    // Load all declarations from output2
    let decls = load_all_declarations()?;
    println!("📊 Loaded {} declarations", decls.len());
    
    // Create eigenmatrix
    let eigenmatrix = create_eigenmatrix(&decls)?;
    
    // Analyze and report
    analyze_eigenmatrix(&eigenmatrix, &decls);
    
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

fn create_eigenmatrix(decls: &[Declaration]) -> Result<Vec<Vec<f64>>> {
    let n = decls.len();
    let mut matrix = vec![vec![0.0; n]; n];
    
    println!("🧮 Computing {}x{} eigenmatrix...", n, n);
    
    for i in 0..n {
        for j in 0..n {
            if i == j {
                matrix[i][j] = 1.0; // Self-similarity
            } else {
                matrix[i][j] = calculate_similarity(&decls[i].content, &decls[j].content);
            }
        }
        
        if i % 100 == 0 {
            println!("   Processed {}/{} rows", i, n);
        }
    }
    
    Ok(matrix)
}

fn calculate_similarity(content1: &str, content2: &str) -> f64 {
    let tokens1: Vec<&str> = content1.split_whitespace().collect();
    let tokens2: Vec<&str> = content2.split_whitespace().collect();
    
    // Rust pattern matching
    let rust_patterns = ["HashMap", "Result", "anyhow", "fs::", "std::", "use", "fn", "struct", "impl", "Vec", "String", "Option"];
    let pattern_matches = rust_patterns.iter()
        .filter(|pattern| content1.contains(*pattern) && content2.contains(*pattern))
        .count();
    
    let common_tokens = tokens1.iter()
        .filter(|token| tokens2.contains(token))
        .count();
    
    let total_tokens = (tokens1.len() + tokens2.len()) as f64;
    let token_similarity = if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens };
    
    // Pattern boost
    let pattern_boost = (pattern_matches as f64) * 0.05;
    
    (token_similarity + pattern_boost).min(1.0)
}

fn analyze_eigenmatrix(matrix: &[Vec<f64>], decls: &[Declaration]) {
    println!("\n🎯 Eigenmatrix Analysis Results:");
    println!("═══════════════════════════════");
    
    // Find highest similarities (excluding diagonal)
    let mut similarities = Vec::new();
    for i in 0..matrix.len() {
        for j in (i+1)..matrix[i].len() {
            similarities.push((matrix[i][j], i, j));
        }
    }
    
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    println!("🔝 Top 10 Similar Declaration Pairs:");
    for (score, i, j) in similarities.iter().take(10) {
        println!("   {:.1}% - {} ↔ {}", score * 100.0, 
                 decls[*i].name.chars().take(40).collect::<String>(),
                 decls[*j].name.chars().take(40).collect::<String>());
    }
    
    // Compute eigenvalues approximation (sum of rows)
    println!("\n🧮 Eigenvalue Approximations (Row Sums):");
    let mut eigenvals: Vec<(f64, usize)> = matrix.iter().enumerate()
        .map(|(i, row)| (row.iter().sum::<f64>(), i))
        .collect();
    eigenvals.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    for (eigenval, i) in eigenvals.iter().take(5) {
        println!("   λ={:.2} - {}", eigenval, 
                 decls[*i].name.chars().take(50).collect::<String>());
    }
    
    // Matrix statistics
    let total_entries = matrix.len() * matrix[0].len();
    let non_diagonal = total_entries - matrix.len();
    let avg_similarity: f64 = similarities.iter().map(|(s, _, _)| s).sum::<f64>() / similarities.len() as f64;
    
    println!("\n📈 Matrix Statistics:");
    println!("   Dimensions: {}x{}", matrix.len(), matrix[0].len());
    println!("   Total comparisons: {}", non_diagonal);
    println!("   Average similarity: {:.1}%", avg_similarity * 100.0);
    println!("   Max similarity: {:.1}%", similarities[0].0 * 100.0);
}

#[derive(Debug)]
struct Declaration {
    name: String,
    content: String,
    path: String,
}
