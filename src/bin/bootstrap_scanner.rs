use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use split_decls_rs::{
    rdf_url_blob::RdfUrlBlob,
    output2_macro_system::{Output2MacroSystem, MacroDeclaration},
};

#[derive(Debug, Clone)]
struct CodePattern {
    pattern: String,
    locations: Vec<String>,
    similarity_score: f64,
}

#[derive(Debug)]
struct DuplicateScanner {
    macros: HashMap<String, MacroDeclaration>,
    patterns: HashMap<String, CodePattern>,
}

impl DuplicateScanner {
    fn new() -> Result<Self> {
        let system = Output2MacroSystem::import_from_output2().unwrap_or_else(|_| {
            Output2MacroSystem {
                macros: HashMap::new(),
                interpreter: split_decls_rs::output2_macro_system::LispInterpreter::new(),
            }
        });
        
        Ok(DuplicateScanner {
            macros: system.macros,
            patterns: HashMap::new(),
        })
    }
    
    fn scan_wrapped_output(&mut self, output_path: &str) -> Result<()> {
        println!("🔍 Scanning wrapped output: {}", output_path);
        
        if !Path::new(output_path).exists() {
            println!("📁 Creating output directory structure...");
            return Ok(());
        }
        
        // Scan all wrapped crates
        for entry in fs::read_dir(output_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                if crate_name.starts_with("wrapped-") {
                    self.scan_crate(&entry.path())?;
                }
            }
        }
        
        self.analyze_duplicates()?;
        Ok(())
    }
    
    fn scan_crate(&mut self, crate_path: &Path) -> Result<()> {
        let crate_name = crate_path.file_name().unwrap().to_string_lossy();
        println!("📦 Scanning crate: {}", crate_name);
        
        let decls_path = crate_path.join("src/decls");
        if !decls_path.exists() {
            return Ok(());
        }
        
        // Scan all declaration files
        for entry in fs::read_dir(decls_path)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                self.scan_declaration_file(&entry.path())?;
            }
        }
        
        Ok(())
    }
    
    fn scan_declaration_file(&mut self, file_path: &Path) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        let file_name = file_path.to_string_lossy().to_string();
        
        // Extract key patterns from the content
        let patterns = self.extract_patterns(&content);
        
        for pattern in patterns {
            // Check if this pattern matches any of our loaded macros
            for (macro_name, macro_decl) in &self.macros {
                let similarity = self.calculate_similarity(&pattern, &macro_decl.content);
                
                if similarity > 0.7 { // 70% similarity threshold
                    let pattern_key = format!("{}_{}", macro_name, self.hash_pattern(&pattern));
                    
                    self.patterns.entry(pattern_key.clone())
                        .or_insert_with(|| CodePattern {
                            pattern: pattern.clone(),
                            locations: Vec::new(),
                            similarity_score: similarity,
                        })
                        .locations.push(file_name.clone());
                        
                    println!("🎯 Found pattern match: {} in {} ({}% similar)", 
                             macro_name, file_name, (similarity * 100.0) as u32);
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_patterns(&self, content: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Extract function signatures
        for line in content.lines() {
            if line.trim().starts_with("pub fn ") || line.trim().starts_with("fn ") {
                if let Some(end) = line.find('{') {
                    patterns.push(line[..end].trim().to_string());
                } else {
                    patterns.push(line.trim().to_string());
                }
            }
            
            // Extract struct definitions
            if line.trim().starts_with("pub struct ") || line.trim().starts_with("struct ") {
                patterns.push(line.trim().to_string());
            }
            
            // Extract macro calls
            if line.contains("!") && (line.contains("macro_rules!") || line.contains("println!") || line.contains("format!")) {
                patterns.push(line.trim().to_string());
            }
        }
        
        patterns
    }
    
    fn calculate_similarity(&self, pattern1: &str, pattern2: &str) -> f64 {
        // Simple similarity based on common tokens
        let tokens1: Vec<&str> = pattern1.split_whitespace().collect();
        let tokens2: Vec<&str> = pattern2.split_whitespace().collect();
        
        let mut common_tokens = 0;
        for token1 in &tokens1 {
            if tokens2.contains(token1) {
                common_tokens += 1;
            }
        }
        
        let max_tokens = tokens1.len().max(tokens2.len());
        if max_tokens == 0 {
            0.0
        } else {
            common_tokens as f64 / max_tokens as f64
        }
    }
    
    fn hash_pattern(&self, pattern: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        pattern.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    fn analyze_duplicates(&self) -> Result<()> {
        println!("\n📊 Duplicate Analysis Results:");
        println!("═══════════════════════════════");
        
        let mut duplicate_count = 0;
        for (pattern_key, pattern) in &self.patterns {
            if pattern.locations.len() > 1 {
                duplicate_count += 1;
                println!("\n🔄 Duplicate Pattern #{}: {}", duplicate_count, pattern_key);
                println!("   📝 Pattern: {}", pattern.pattern.chars().take(80).collect::<String>());
                println!("   🎯 Similarity: {:.1}%", pattern.similarity_score * 100.0);
                println!("   📍 Found in {} locations:", pattern.locations.len());
                
                for (i, location) in pattern.locations.iter().enumerate() {
                    println!("      {}. {}", i + 1, location);
                }
            }
        }
        
        if duplicate_count == 0 {
            println!("✅ No significant duplicates found");
        } else {
            println!("\n📈 Summary: Found {} duplicate patterns", duplicate_count);
        }
        
        Ok(())
    }
    
    fn generate_repl_commands(&self) -> Result<()> {
        println!("\n🤖 Generated REPL Commands:");
        println!("═══════════════════════════════");
        
        for (pattern_key, pattern) in &self.patterns {
            if pattern.locations.len() > 1 {
                println!("# Load and analyze pattern: {}", pattern_key);
                println!("set pattern_{} = \"{}\"", 
                         pattern_key.replace("-", "_"), 
                         pattern.pattern.replace("\"", "\\\""));
                println!("macro analyze_{} println!(\"Analyzing: {{}}\", pattern_{})", 
                         pattern_key.replace("-", "_"),
                         pattern_key.replace("-", "_"));
                println!();
            }
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("🚀 Bootstrap Duplicate Code Scanner");
    println!("Using loaded macros as search keys...\n");
    
    let mut scanner = DuplicateScanner::new()?;
    
    // Scan the wrapped output directory
    scanner.scan_wrapped_output("output2")?;
    
    // Generate REPL commands for further analysis
    scanner.generate_repl_commands()?;
    
    println!("\n✨ Scan complete! Use the generated REPL commands to explore duplicates.");
    
    Ok(())
}
