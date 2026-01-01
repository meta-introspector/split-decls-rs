use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;

fn parse_rlib_symbols(rlib_path: &str) -> HashMap<String, i32> {
    let mut symbols = HashMap::new();
    
    // Use nm to get symbols with more detail
    let output = Command::new("nm")
        .args(&["-D", "-g", "--defined-only", rlib_path])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let symbol_type = parts[1];
            *symbols.entry(symbol_type.to_string()).or_insert(0) += 1;
        }
    }
    
    symbols
}

fn analyze_elf_instructions(rlib_path: &str, chunk_size: usize) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut instruction_profile = HashMap::new();
    
    // Create temp directory for extraction
    let temp_dir = format!("samples/elf_{}decls", chunk_size);
    std::fs::create_dir_all(&temp_dir)?;
    
    // Extract archive contents
    let _ = Command::new("ar")
        .args(&["x", rlib_path])
        .current_dir(&temp_dir)
        .output()?;
    
    // Disassemble object files to get instruction profile
    let objdump_output = Command::new("find")
        .args(&[&temp_dir, "-name", "*.o", "-exec", "objdump", "-d", "{}", ";"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    // Parse disassembly for instruction types
    let disasm = String::from_utf8_lossy(&objdump_output.stdout);
    for line in disasm.lines() {
        if line.contains(":") && line.contains("\t") {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let instruction = parts[2].split_whitespace().next().unwrap_or("");
                if !instruction.is_empty() {
                    *instruction_profile.entry(instruction.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    // Get memory sections info
    let readelf_sections = Command::new("find")
        .args(&[&temp_dir, "-name", "*.o", "-exec", "readelf", "-S", "{}", ";"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    // Save detailed analysis
    let analysis_file = format!("samples/instruction_profile_{}decls.txt", chunk_size);
    let mut analysis_content = String::new();
    analysis_content.push_str("=== INSTRUCTION PROFILE ===\n");
    
    let mut sorted_instructions: Vec<_> = instruction_profile.iter().collect();
    sorted_instructions.sort_by(|a, b| b.1.cmp(a.1));
    
    for (instruction, count) in &sorted_instructions {
        analysis_content.push_str(&format!("{:>6} {}\n", count, instruction));
    }
    
    analysis_content.push_str("\n=== MEMORY SECTIONS ===\n");
    analysis_content.push_str(&String::from_utf8_lossy(&readelf_sections.stdout));
    
    analysis_content.push_str("\n=== DISASSEMBLY ===\n");
    analysis_content.push_str(&disasm);
    
    fs::write(&analysis_file, analysis_content)?;
    
    Ok(instruction_profile)
}
    // Create temp directory for extraction
    let temp_dir = format!("samples/elf_{}decls", chunk_size);
    std::fs::create_dir_all(&temp_dir)?;
    
    // Extract archive contents
    let extract_output = Command::new("ar")
        .args(&["x", rlib_path])
        .current_dir(&temp_dir)
        .output()?;
    
    if extract_output.status.success() {
        // Get ELF headers from object files
        let readelf_output = Command::new("find")
            .args(&[&temp_dir, "-name", "*.o", "-exec", "readelf", "-h", "{}", ";"])
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });
        
        // Save ELF analysis
        let elf_file = format!("samples/elf_{}decls.txt", chunk_size);
        let mut elf_content = String::new();
        elf_content.push_str("=== ARCHIVE CONTENTS ===\n");
        elf_content.push_str(&String::from_utf8_lossy(&extract_output.stdout));
        elf_content.push_str("\n=== ELF HEADERS ===\n");
        elf_content.push_str(&String::from_utf8_lossy(&readelf_output.stdout));
        
        // Get symbol tables
        let symbols_output = Command::new("find")
            .args(&[&temp_dir, "-name", "*.o", "-exec", "readelf", "-s", "{}", ";"])
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });
        
        elf_content.push_str("\n=== SYMBOL TABLES ===\n");
        elf_content.push_str(&String::from_utf8_lossy(&symbols_output.stdout));
        
        fs::write(&elf_file, elf_content)?;
        println!("    📊 ELF analysis saved: {}", elf_file);
    }
    
    Ok(())
}
    let mut symbols = HashMap::new();
    
    if let Ok(output) = Command::new("nm")
        .args(&["target/debug/libsplit_decls_genesis.rlib"])
        .output() 
    {
        let nm_str = String::from_utf8_lossy(&output.stdout);
        for line in nm_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let symbol_type = parts[1];
                *symbols.entry(symbol_type.to_string()).or_insert(0) += 1;
            }
        }
    }
    symbols
}

fn analyze_source_histogram(content: &str) -> HashMap<String, i32> {
    let mut histogram = HashMap::new();
    
    // Extract Rust keywords and identifiers
    for word in content.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
            .to_lowercase();
        
        if clean_word.len() > 2 && !clean_word.chars().all(|c| c.is_numeric()) {
            *histogram.entry(clean_word).or_insert(0) += 1;
        }
    }
    
    histogram
}
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn get_target_files() -> Vec<String> {
    let output = Command::new("ls")
        .args(&["-latr", "target/debug/"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: ExitStatus::from_raw(256), // 1 << 8
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn compile_with_chunk(tree_content: &str, chunk_size: usize) -> Result<(u64, u64, HashMap<String, i32>, HashMap<String, i32>), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = tree_content.lines().collect();
    let partial_content = lines.iter().take(chunk_size).cloned().collect::<Vec<_>>().join("\n");
    
    // Analyze source histogram
    let source_histogram = analyze_source_histogram(&partial_content);
    
    // Generate source with chunk of declarations
    let mut full_content = String::new();
    full_content.push_str("#![recursion_limit = \"256\"]\n");
    full_content.push_str("#![allow(internal_features)]\n");
    full_content.push_str("#![feature(rustc_private)]\n\n");
    full_content.push_str("include!(\"wrap_types.rs\");\n\n");
    full_content.push_str(&partial_content);
    
    fs::write("src/current.rs", &full_content)?;
    
    // Save generated file for analysis
    let chunk_name = format!("samples/generated_{}decls.rs", chunk_size);
    fs::create_dir_all("samples")?;
    fs::write(&chunk_name, &full_content)?;
    
    let source_size = get_file_size("src/current.rs");
    
    // Clean previous build artifacts
    let _ = Command::new("cargo").args(&["clean"]).output();
    
    // Compile with verbose output and internal dumps
    let output = Command::new("cargo")
        .args(&["build", "--lib", "-v"])
        .env("RUSTC_LOG", "debug")
        .env("RUST_LOG", "debug")
        .output()?;
    
    // Try to get MIR dump
    let mir_output = Command::new("cargo")
        .args(&["rustc", "--lib", "--", "-Z", "dump-mir=all", "-Z", "dump-mir-dir=samples/mir"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(256),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    // Try to get HIR dump
    let hir_output = Command::new("cargo")
        .args(&["rustc", "--lib", "--", "-Z", "unpretty=hir"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(256),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    // Save rustc logs
    let log_name = format!("samples/rustc_{}decls.log", chunk_size);
    let mut log_content = String::new();
    log_content.push_str("=== STDOUT ===\n");
    log_content.push_str(&String::from_utf8_lossy(&output.stdout));
    log_content.push_str("\n=== STDERR ===\n");
    log_content.push_str(&String::from_utf8_lossy(&output.stderr));
    fs::write(&log_name, log_content.clone())?;
    
    // Analyze log histogram
    let log_histogram = analyze_source_histogram(&log_content);
    
    if output.status.success() {
        let binary_size = get_file_size("target/debug/libsplit_decls_genesis.rlib");
        let symbols = parse_rlib_symbols("target/debug/libsplit_decls_genesis.rlib");
        
        // Save binary for analysis
        let binary_name = format!("samples/binary_{}decls.rlib", chunk_size);
        let _ = fs::copy("target/debug/libsplit_decls_genesis.rlib", &binary_name);
        
        // Analyze ELF structure
        let _ = analyze_rlib_elf("target/debug/libsplit_decls_genesis.rlib", chunk_size);
        
        println!("    💾 Saved: {}, {}, {}", chunk_name, log_name, binary_name);
        
        Ok((source_size, binary_size, symbols, source_histogram))
    } else {
        println!("  ❌ FAILED - Compilation error with {} declarations", chunk_size);
        Err("Compilation failed".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <rustc_main_symbol>", args[0]);
        println!("Example: {} rustc_driver::main", args[0]);
        return Ok(());
    }
    
    let target = &args[1];
    println!("🎯 Target: {}", target);
    
    // Convert symbol to tree filename
    let tree_name = target.replace("::", "_").replace(".", "_");
    let tree_path = format!("src/bin_trees/{}.rs", tree_name);
    
    if !Path::new(&tree_path).exists() {
        println!("❌ No module tree found for: {}", target);
        return Ok(());
    }
    
    println!("📋 Loading module tree: {}", tree_path);
    let tree_content = fs::read_to_string(&tree_path)?;
    
    println!("\n📊 PROGRESSIVE DEPENDENCY ANALYSIS");
    println!("==================================");
    
    let lines: Vec<&str> = tree_content.lines().collect();
    let chunks = vec![10, 20, 30, 40]; // Test chunks of 10, 20, 30, 40 declarations
    
    for &chunk_size in &chunks {
        if chunk_size <= lines.len() {
            println!("\n🔧 Testing with {} declarations:", chunk_size);
            
            match compile_with_chunk(&tree_content, chunk_size) {
                Ok((source_size, binary_size, symbols, source_histogram)) => {
                    let total_symbols: i32 = symbols.values().sum();
                    
                    println!("  ✅ SUCCESS");
                    println!("    Source size:  {:>8} bytes", source_size);
                    println!("    Binary size:  {:>8} bytes", binary_size);
                    println!("    Total symbols: {:>7}", total_symbols);
                    
                    // Top symbol types
                    let mut sorted: Vec<_> = symbols.iter().collect();
                    sorted.sort_by(|a, b| b.1.cmp(a.1));
                    println!("    Top symbols:");
                    for (symbol_type, count) in sorted.iter().take(3) {
                        println!("      {:>3} {:>6}", symbol_type, count);
                    }
                    
                    // Top source terms
                    let mut source_sorted: Vec<_> = source_histogram.iter().collect();
                    source_sorted.sort_by(|a, b| b.1.cmp(a.1));
                    println!("    Top source terms:");
                    for (term, count) in source_sorted.iter().take(5) {
                        println!("      {:>3} {}", count, term);
                    }
                }
                Err(_) => {
                    println!("  ❌ FAILED - Compilation error with {} declarations", chunk_size);
                }
            }
        }
    }
    
    Ok(())
}
