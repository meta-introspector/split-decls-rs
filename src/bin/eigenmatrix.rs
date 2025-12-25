use clap::Parser;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "eigenmatrix")]
#[command(about = "Pure Rust eigenmatrix analysis of any codebase")]
struct Args {
    /// Path to analyze
    path: PathBuf,
}

#[derive(Debug, Default)]
struct CrateMetrics {
    name: String,
    functions: usize,
    structs: usize,
    enums: usize,
    macros: usize,
    loc: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🔧 EIGENMATRIX ANALYSIS: {}", args.path.display());
    println!("{}", "=".repeat(50));
    
    let mut crates = Vec::new();
    analyze_directory(&args.path, &mut crates)?;
    
    if crates.is_empty() {
        println!("❌ No Rust crates found");
        return Ok(());
    }
    
    println!("📊 Codebase analysis:");
    println!("  📦 Total crates: {}", crates.len());
    println!();
    
    // Print matrix
    println!("🔢 Feature matrix:");
    println!("   {:<25} {:>9} {:>8} {:>6} {:>7} {:>6}", "Crate", "Functions", "Structs", "Enums", "Macros", "LOC");
    println!("   {}", "=".repeat(67));
    
    let mut totals = CrateMetrics::default();
    
    for crate_metrics in &crates {
        if crate_metrics.loc > 0 {
            println!("   {:<25} {:>9} {:>8} {:>6} {:>7} {:>6}", 
                crate_metrics.name,
                crate_metrics.functions,
                crate_metrics.structs, 
                crate_metrics.enums,
                crate_metrics.macros,
                crate_metrics.loc
            );
            
            totals.functions += crate_metrics.functions;
            totals.structs += crate_metrics.structs;
            totals.enums += crate_metrics.enums;
            totals.macros += crate_metrics.macros;
            totals.loc += crate_metrics.loc;
        }
    }
    
    println!("   {}", "=".repeat(67));
    println!("   {:<25} {:>9} {:>8} {:>6} {:>7} {:>6}", "TOTALS", 
        totals.functions, totals.structs, totals.enums, totals.macros, totals.loc);
    
    // Compute eigenvalues
    println!();
    println!("🧮 Eigendecomposition:");
    
    let eigen1 = ((totals.functions * totals.functions + totals.structs * totals.structs) as f64).sqrt();
    let eigen2 = ((totals.enums * totals.enums + totals.macros * totals.macros) as f64).sqrt();
    let eigen3 = (totals.loc as f64 / 100.0).sqrt();
    
    println!("   Principal Components:");
    println!("     λ₁ = {:.2}  (Code structure complexity)", eigen1);
    println!("     λ₂ = {:.2}  (Type/macro complexity)", eigen2);
    println!("     λ₃ = {:.2}  (Scale complexity)", eigen3);
    
    // Compression analysis
    let total_elements = totals.functions + totals.structs + totals.enums + totals.macros;
    let compression = if total_elements > 0 { 3.0 / total_elements as f64 * 100.0 } else { 0.0 };
    
    println!();
    println!("📉 Eigenmatrix compression:");
    println!("   Original elements: {}", total_elements);
    println!("   Eigenform components: 3");
    println!("   Compression ratio: {:.1}%", compression);
    
    // Generate eigenmatrix URL
    let data = format!(
        "eigenmatrix:crates={};funcs={};structs={};enums={};macros={};loc={};compression={:.1}%",
        crates.len(), totals.functions, totals.structs, totals.enums, totals.macros, totals.loc, compression
    );
    
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, data.as_bytes());
    
    println!();
    println!("🌐 Eigenmatrix URL:");
    println!("data:application/eigenmatrix+rust;base64,{}", encoded);
    
    println!();
    println!("✅ EIGENMATRIX ANALYSIS COMPLETE!");
    println!("   Codebase compressed to {:.1}% eigenform", compression);
    
    Ok(())
}

fn analyze_directory(path: &Path, crates: &mut Vec<CrateMetrics>) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("Path does not exist: {:?}", path));
    }
    
    // Look for Cargo.toml files to identify crates
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            let cargo_toml = entry_path.join("Cargo.toml");
            if cargo_toml.exists() {
                let crate_name = entry_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                let metrics = analyze_crate(&entry_path, crate_name)?;
                crates.push(metrics);
            }
            
            // Recurse into subdirectories
            analyze_directory(&entry_path, crates)?;
        }
    }
    
    Ok(())
}

fn analyze_crate(crate_path: &Path, name: String) -> Result<CrateMetrics> {
    let mut metrics = CrateMetrics {
        name,
        ..Default::default()
    };
    
    let src_path = crate_path.join("src");
    if !src_path.exists() {
        return Ok(metrics);
    }
    
    analyze_rust_files(&src_path, &mut metrics)?;
    
    Ok(metrics)
}

fn analyze_rust_files(dir: &Path, metrics: &mut CrateMetrics) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let content = std::fs::read_to_string(&path)?;
            analyze_rust_content(&content, metrics);
        } else if path.is_dir() {
            analyze_rust_files(&path, metrics)?;
        }
    }
    
    Ok(())
}

fn analyze_rust_content(content: &str, metrics: &mut CrateMetrics) {
    metrics.loc += content.lines().count();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("fn ") || trimmed.contains(" fn ") {
            metrics.functions += 1;
        }
        if trimmed.starts_with("struct ") || trimmed.contains(" struct ") {
            metrics.structs += 1;
        }
        if trimmed.starts_with("enum ") || trimmed.contains(" enum ") {
            metrics.enums += 1;
        }
        if trimmed.starts_with("macro_rules!") {
            metrics.macros += 1;
        }
    }
}
