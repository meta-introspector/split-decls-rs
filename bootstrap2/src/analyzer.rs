use anyhow::Result;
use std::fs;
use std::path::Path;

/// Declaration file analyzer for Bootstrap2 auditing
pub struct DeclarationAnalyzer {
    decls_path: std::path::PathBuf,
}

impl DeclarationAnalyzer {
    pub fn new(decls_path: &Path) -> Self {
        Self {
            decls_path: decls_path.to_path_buf(),
        }
    }

    /// Analyze all bootstrap-related declarations
    pub fn analyze_bootstrap_declarations(&self) -> Result<Vec<String>> {
        let mut bootstrap_files = Vec::new();
        
        for entry in fs::read_dir(&self.decls_path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            if file_name.contains("bootstrap") {
                bootstrap_files.push(file_name);
            }
        }
        
        Ok(bootstrap_files)
    }

    /// Analyze function declarations by reading file contents
    pub fn analyze_function_declarations(&self, pattern: &str) -> Result<Vec<(String, String)>> {
        let mut functions = Vec::new();
        
        for entry in fs::read_dir(&self.decls_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(&path)?;
                if content.contains(pattern) {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Extract function signature
                    if let Some(fn_line) = content.lines().find(|line| line.contains("pub fn")) {
                        functions.push((file_name, fn_line.trim().to_string()));
                    }
                }
            }
        }
        
        Ok(functions)
    }

    /// Get statistics about generated declarations
    pub fn get_statistics(&self) -> Result<DeclarationStats> {
        let mut stats = DeclarationStats::default();
        
        for entry in fs::read_dir(&self.decls_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "rs") {
                stats.total_files += 1;
                
                let content = fs::read_to_string(&path)?;
                if content.contains("pub fn") {
                    stats.function_files += 1;
                }
                if content.contains("pub struct") {
                    stats.struct_files += 1;
                }
                if content.contains("pub enum") {
                    stats.enum_files += 1;
                }
            }
        }
        
        Ok(stats)
    }
}

#[derive(Default, Debug)]
pub struct DeclarationStats {
    pub total_files: usize,
    pub function_files: usize,
    pub struct_files: usize,
    pub enum_files: usize,
}
