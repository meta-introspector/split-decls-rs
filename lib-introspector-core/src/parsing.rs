/// Core parsing functionality for Rust source code introspection
/// 
/// This module provides the fundamental parsing capabilities extracted from build.rs
/// to test and validate Rust source code transformations.

use syn::{parse_file, File};

/// Test if a given Rust source code string can be successfully parsed
/// 
/// # Arguments
/// * `content` - The Rust source code to parse
/// 
/// # Returns
/// * `(bool, Option<String>)` - (success, error_message)
pub fn test_parse(content: &str) -> (bool, Option<String>) {
    match parse_file(content) {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    }
}

/// Get detailed parse error with comprehensive diagnostics
pub fn detailed_parse_error(content: &str) -> String {
    match parse_file(content) {
        Ok(_) => "✅ Parses successfully".to_string(),
        Err(e) => {
            let mut report = String::new();
            
            // Basic error info
            report.push_str(&format!("❌ SYN PARSE ERROR: {}\n", e));
            report.push_str(&format!("📍 Error span: {:?}\n", e.span()));
            
            // Try to extract line/column info
            let lines: Vec<&str> = content.lines().collect();
            report.push_str(&format!("📄 Total lines: {}\n", lines.len()));
            report.push_str(&format!("📏 Total chars: {}\n", content.len()));
            
            // Show context around error if possible
            let error_str = e.to_string();
            if let Some(line_info) = extract_line_info(&error_str) {
                report.push_str(&format!("🎯 Error location: {}\n", line_info));
                
                // Try to show context around the error
                if let Some((line_num, col_num)) = parse_line_col(&line_info) {
                    if line_num > 0 && line_num <= lines.len() {
                        report.push_str("\n📋 CONTEXT:\n");
                        
                        // Show 3 lines before
                        for i in (line_num.saturating_sub(3))..line_num {
                            if i > 0 && i <= lines.len() {
                                report.push_str(&format!("{:4} | {}\n", i, lines[i-1]));
                            }
                        }
                        
                        // Show error line with pointer
                        if line_num <= lines.len() {
                            report.push_str(&format!("{:4} > {}\n", line_num, lines[line_num-1]));
                            if col_num > 0 {
                                report.push_str(&format!("     {}{}^\n", " ".repeat(col_num.saturating_sub(1)), " ".repeat(0)));
                            }
                        }
                        
                        // Show 3 lines after
                        for i in (line_num+1)..=(line_num+3).min(lines.len()) {
                            if i <= lines.len() {
                                report.push_str(&format!("{:4} | {}\n", i, lines[i-1]));
                            }
                        }
                    }
                }
            }
            
            // Character-by-character analysis around potential error
            report.push_str("\n🔍 CHARACTER ANALYSIS:\n");
            let chars: Vec<char> = content.chars().collect();
            for (i, &ch) in chars.iter().enumerate().take(200) {
                if ch.is_control() && ch != '\n' && ch != '\t' {
                    report.push_str(&format!("⚠️  Control char at pos {}: {:?} (U+{:04X})\n", i, ch, ch as u32));
                }
                if !ch.is_ascii() {
                    report.push_str(&format!("🌐 Non-ASCII at pos {}: {:?} (U+{:04X})\n", i, ch, ch as u32));
                }
            }
            
            // Look for common problematic patterns
            report.push_str("\n🔎 PATTERN ANALYSIS:\n");
            if content.contains("# [") {
                report.push_str("⚠️  Found spaced attributes: '# [' (should be '#[')\n");
            }
            if content.contains("env!") {
                report.push_str("📝 Found env! macro calls\n");
            }
            if content.contains("include!") {
                report.push_str("📝 Found include! macro calls\n");
            }
            if content.contains("concat!") {
                report.push_str("📝 Found concat! macro calls\n");
            }
            if content.contains("cfg!") {
                report.push_str("📝 Found cfg! macro calls\n");
            }
            
            // Test with rustc for comparison
            report.push_str("\n🔍 RUSTC VALIDATION:\n");
            
            use std::fs;
            use std::process::Command;
            
            let temp_file = "/tmp/detailed_parse_test.rs";
            let full_content = format!(
                "#![allow(unused)]\n\
                 #![feature(try_blocks)]\n\
                 #![feature(error_reporter)]\n\
                 #![feature(rustc_private)]\n\
                 // Comprehensive stub dependencies\n\
                 mod rustc_error_messages {{\n\
                     pub type FluentArgs = std::collections::HashMap<String, String>;\n\
                     pub struct LazyFallbackBundle;\n\
                 }}\n\
                 mod tracing {{\n\
                     pub fn debug(_: &str) {{}}\n\
                     pub fn trace(_: &str) {{}}\n\
                 }}\n\
                 mod error {{\n\
                     pub struct TranslateError;\n\
                     pub struct TranslateErrorKind;\n\
                 }}\n\
                 mod snippet {{ pub struct Style; }}\n\
                 struct DiagArg;\n\
                 struct DiagMessage;\n\
                 struct FluentBundle;\n\
                 fn fallback_fluent_bundle(_: &str, _: Vec<String>) -> FluentBundle {{ FluentBundle }}\n\
                 use std::env;\n\
                 use std::error::Report;\n\
                 \n{}", 
                content
            );
            
            if let Ok(()) = fs::write(temp_file, &full_content) {
                let output = Command::new("rustc")
                    .args(&["--crate-type", "lib", temp_file, "-o", "/tmp/detailed_parse_test", "--error-format=human"])
                    .output();
                
                match output {
                    Ok(result) => {
                        if !result.status.success() {
                            let stderr = String::from_utf8_lossy(&result.stderr);
                            report.push_str("📍 Rustc errors:\n");
                            for line in stderr.lines().take(20) {
                                report.push_str(&format!("   {}\n", line));
                            }
                            if stderr.lines().count() > 20 {
                                report.push_str("   ... (truncated)\n");
                            }
                        } else {
                            report.push_str("✅ Rustc compiles successfully - this is a syn-specific parsing issue\n");
                        }
                    }
                    Err(e) => {
                        report.push_str(&format!("⚠️ Could not run rustc: {}\n", e));
                    }
                }
                
                // Clean up
                let _ = fs::remove_file(temp_file);
                let _ = fs::remove_file("/tmp/detailed_parse_test");
            } else {
                report.push_str("⚠️ Could not write temp file for rustc test\n");
            }
            
            // Try minimal parsing to isolate the issue
            report.push_str("\n🧪 MINIMAL PARSING TESTS:\n");
            
            // Test just the first few lines
            let first_10_lines: String = content.lines().take(10).collect::<Vec<_>>().join("\n");
            match parse_file(&first_10_lines) {
                Ok(_) => report.push_str("✅ First 10 lines parse OK\n"),
                Err(e) => report.push_str(&format!("❌ First 10 lines fail: {}\n", e)),
            }
            
            // Test with minimal stub
            let minimal_test = format!("fn test() {{\n{}\n}}", content.lines().take(5).collect::<Vec<_>>().join("\n"));
            match parse_file(&minimal_test) {
                Ok(_) => report.push_str("✅ Wrapped in function parses OK\n"),
                Err(e) => report.push_str(&format!("❌ Wrapped in function fails: {}\n", e)),
            }
            
            report
        }
    }
}

/// Extract line information from error message
fn extract_line_info(error_msg: &str) -> Option<String> {
    // Look for patterns like "at line X column Y" or similar
    if let Some(pos) = error_msg.find("line") {
        if let Some(end) = error_msg[pos..].find('\n').or(Some(error_msg.len() - pos)) {
            return Some(error_msg[pos..pos + end].to_string());
        }
    }
    None
}

/// Parse line and column numbers from error info
fn parse_line_col(line_info: &str) -> Option<(usize, usize)> {
    // Very basic parsing - could be enhanced
    let parts: Vec<&str> = line_info.split_whitespace().collect();
    let mut line_num = 0;
    let mut col_num = 0;
    
    for (i, part) in parts.iter().enumerate() {
        if *part == "line" && i + 1 < parts.len() {
            if let Ok(num) = parts[i + 1].parse::<usize>() {
                line_num = num;
            }
        }
        if *part == "column" && i + 1 < parts.len() {
            if let Ok(num) = parts[i + 1].parse::<usize>() {
                col_num = num;
            }
        }
    }
    
    if line_num > 0 {
        Some((line_num, col_num))
    } else {
        None
    }
}

/// Parse Rust source code and return the AST
/// 
/// # Arguments  
/// * `content` - The Rust source code to parse
/// 
/// # Returns
/// * `Result<File, syn::Error>` - The parsed AST or error
pub fn parse_rust_source(content: &str) -> Result<File, syn::Error> {
    parse_file(content)
}
