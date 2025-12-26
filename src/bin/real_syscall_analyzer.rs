use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct RealSyscallAnalyzer {
    pub syscall_patterns: HashMap<String, Vec<String>>,
    pub file_locations: HashMap<String, Vec<String>>,
    pub total_files_scanned: usize,
}

impl RealSyscallAnalyzer {
    pub fn new() -> Self {
        Self {
            syscall_patterns: HashMap::new(),
            file_locations: HashMap::new(),
            total_files_scanned: 0,
        }
    }
    
    pub fn scan_directory(&mut self, dir: &Path) -> Result<()> {
        println!("🔍 Scanning directory: {:?}", dir);
        
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() && 
               entry.path().extension().map_or(false, |ext| ext == "rs") {
                
                self.scan_file(entry.path())?;
                self.total_files_scanned += 1;
                
                if self.total_files_scanned % 100 == 0 {
                    println!("  Scanned {} files...", self.total_files_scanned);
                }
            }
        }
        
        Ok(())
    }
    
    fn scan_file(&mut self, file_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Define syscall patterns to search for
        let patterns = [
            ("std::fs::", "filesystem"),
            ("std::process::", "process"),
            ("std::env::", "environment"),
            ("std::io::", "io"),
            ("std::net::", "network"),
            ("libc::", "libc"),
            ("std::time::", "time"),
            ("std::thread::", "thread"),
            ("std::sync::", "sync"),
            ("std::mem::", "memory"),
        ];
        
        for (pattern, category) in &patterns {
            let matches: Vec<&str> = content.matches(pattern).collect();
            if !matches.is_empty() {
                self.syscall_patterns
                    .entry(category.to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("{}: {} occurrences", file_path_str, matches.len()));
                
                self.file_locations
                    .entry(category.to_string())
                    .or_insert_with(Vec::new)
                    .push(file_path_str.clone());
            }
        }
        
        Ok(())
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("📊 Real Syscall Analysis Report\n");
        report.push_str("===============================\n\n");
        
        report.push_str(&format!("📁 Total files scanned: {}\n\n", self.total_files_scanned));
        
        let mut total_syscalls = 0;
        let mut category_counts: Vec<(String, usize)> = Vec::new();
        
        for (category, locations) in &self.syscall_patterns {
            let count = locations.len();
            total_syscalls += count;
            category_counts.push((category.clone(), count));
        }
        
        category_counts.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.push_str("🔍 Syscall Categories Found:\n");
        for (category, count) in &category_counts {
            let percentage = if total_syscalls > 0 {
                (count * 100) as f64 / total_syscalls as f64
            } else {
                0.0
            };
            
            let emoji = match category.as_str() {
                "filesystem" => "📁",
                "process" => "⚡",
                "environment" => "🌍",
                "io" => "💾",
                "network" => "🌐",
                "libc" => "🔧",
                "time" => "⏰",
                "thread" => "🧵",
                "sync" => "🔒",
                "memory" => "🧠",
                _ => "❓",
            };
            
            report.push_str(&format!("   {} {}: {} files ({:.1}%)\n", 
                                   emoji, category, count, percentage));
        }
        
        report.push_str(&format!("\n📈 Total syscall usage found in {} files\n\n", total_syscalls));
        
        // Show top files for each category
        for (category, locations) in &self.syscall_patterns {
            if !locations.is_empty() {
                report.push_str(&format!("📋 Top {} files:\n", category));
                for (i, location) in locations.iter().take(5).enumerate() {
                    report.push_str(&format!("   {}. {}\n", i + 1, location));
                }
                report.push_str("\n");
            }
        }
        
        report
    }
    
    pub fn get_syscall_counts(&self) -> HashMap<String, usize> {
        self.syscall_patterns.iter()
            .map(|(k, v)| (k.clone(), v.len()))
            .collect()
    }
}

fn main() -> Result<()> {
    let mut analyzer = RealSyscallAnalyzer::new();
    
    // Scan current directory
    analyzer.scan_directory(Path::new("."))?;
    
    // Generate and display report
    let report = analyzer.generate_report();
    println!("{}", report);
    
    // Save report to file
    std::fs::write("real_syscall_analysis.md", report)?;
    println!("💾 Report saved to: real_syscall_analysis.md");
    
    // Generate JSON data for other tools
    let counts = analyzer.get_syscall_counts();
    let json = serde_json::to_string_pretty(&counts)?;
    std::fs::write("syscall_counts.json", json)?;
    println!("📊 Data saved to: syscall_counts.json");
    
    Ok(())
}
