use std::process::Command;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Incremental Compile Driver");
    
    let mut stats = CompileStats::new();
    let src_dir = "src";
    
    // Get all processed files sorted by size (smallest first)
    let mut files: Vec<_> = fs::read_dir(src_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "rs") &&
            entry.path().file_name().unwrap().to_str().unwrap().starts_with("processed_")
        })
        .collect();
    
    files.sort_by_key(|entry| entry.metadata().unwrap().len());
    
    println!("📊 Found {} processed files", files.len());
    
    for (i, entry) in files.iter().enumerate() {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        
        print!("[{:3}/{:3}] {:<50} ", i+1, files.len(), 
               &file_name[..file_name.len().min(50)]);
        
        match compile_file(&path) {
            CompileResult::Success => {
                println!("✅");
                stats.success += 1;
            }
            CompileResult::Error(errors) => {
                println!("❌ ({} errors)", errors.len());
                stats.failed += 1;
                
                // Show first error
                if let Some(first_error) = errors.first() {
                    println!("    └─ {}", first_error);
                }
                
                // Track error patterns
                for error in errors {
                    *stats.error_patterns.entry(extract_error_code(&error)).or_insert(0) += 1;
                }
            }
        }
        
        // Show progress every 10 files
        if (i + 1) % 10 == 0 {
            stats.print_progress();
        }
    }
    
    stats.print_final();
    Ok(())
}

struct CompileStats {
    success: usize,
    failed: usize,
    error_patterns: HashMap<String, usize>,
}

impl CompileStats {
    fn new() -> Self {
        Self {
            success: 0,
            failed: 0,
            error_patterns: HashMap::new(),
        }
    }
    
    fn print_progress(&self) {
        let total = self.success + self.failed;
        let success_rate = if total > 0 { (self.success * 100) / total } else { 0 };
        println!("    📈 Progress: {}/{} ({}% success)", self.success, total, success_rate);
    }
    
    fn print_final(&self) {
        println!("\n🎯 Final Results:");
        println!("   ✅ Success: {}", self.success);
        println!("   ❌ Failed:  {}", self.failed);
        
        println!("\n🔍 Top Error Patterns:");
        let mut patterns: Vec<_> = self.error_patterns.iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        for (pattern, count) in patterns.iter().take(5) {
            println!("   {} × {}", count, pattern);
        }
    }
}

enum CompileResult {
    Success,
    Error(Vec<String>),
}

fn compile_file(path: &Path) -> CompileResult {
    let output = Command::new("rustc")
        .args(&[
            "--crate-type", "lib",
            "--allow", "warnings",
            "--edition", "2021",
            "-o", "/tmp/test.rlib",
            path.to_str().unwrap()
        ])
        .output()
        .expect("Failed to run rustc");
    
    if output.status.success() {
        CompileResult::Success
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let errors: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("error:") || line.contains("error[E"))
            .map(|s| s.to_string())
            .collect();
        
        CompileResult::Error(errors)
    }
}

fn extract_error_code(error: &str) -> String {
    if let Some(start) = error.find("error[E") {
        if let Some(end) = error[start..].find(']') {
            return error[start..start + end + 1].to_string();
        }
    }
    "error[UNKNOWN]".to_string()
}
