use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct NodeUsageReport {
    integer_value: i32,
    total_usage_count: usize,
    user_types: HashMap<String, usize>, // type -> count
    relations: HashMap<String, usize>,  // relation type -> count
    ast_contexts: HashMap<String, usize>, // AST context -> count
    crate_distribution: HashMap<String, usize>, // crate -> count
}

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut usage_reports: HashMap<i32, NodeUsageReport> = HashMap::new();
    
    // Analyze all nodes for integer usage patterns
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, node) in nodes {
            analyze_node_usage(symbol, node, &mut usage_reports);
        }
    }
    
    // Sort by total usage
    let mut sorted_reports: Vec<_> = usage_reports.into_values().collect();
    sorted_reports.sort_by(|a, b| b.total_usage_count.cmp(&a.total_usage_count));
    
    // Generate comprehensive report
    println!("🔗 Integer Node Usage & Relations Report");
    println!("========================================\n");
    
    for (i, report) in sorted_reports.iter().take(20).enumerate() {
        println!("{}. Integer: {} (Used {} times)", i+1, report.integer_value, report.total_usage_count);
        
        // Top user types
        let mut user_types: Vec<_> = report.user_types.iter().collect();
        user_types.sort_by(|a, b| b.1.cmp(a.1));
        println!("   👥 User Types: {}", 
            user_types.iter().take(3)
                .map(|(t, c)| format!("{}({})", t, c))
                .collect::<Vec<_>>().join(", "));
        
        // Top relations
        let mut relations: Vec<_> = report.relations.iter().collect();
        relations.sort_by(|a, b| b.1.cmp(a.1));
        println!("   🔗 Relations: {}", 
            relations.iter().take(3)
                .map(|(r, c)| format!("{}({})", r, c))
                .collect::<Vec<_>>().join(", "));
        
        // Top AST contexts
        let mut contexts: Vec<_> = report.ast_contexts.iter().collect();
        contexts.sort_by(|a, b| b.1.cmp(a.1));
        println!("   🌳 AST Contexts: {}", 
            contexts.iter().take(3)
                .map(|(c, n)| format!("{}({})", c, n))
                .collect::<Vec<_>>().join(", "));
        
        // Crate distribution
        println!("   📦 Crates: {} different crates", report.crate_distribution.len());
        println!();
    }
    
    // Save detailed report
    let report_json = serde_json::to_string_pretty(&sorted_reports)?;
    fs::write("integer_usage_relations_report.json", report_json)?;
    
    // Summary statistics
    let total_integers = sorted_reports.len();
    let total_usages: usize = sorted_reports.iter().map(|r| r.total_usage_count).sum();
    let avg_usage = if total_integers > 0 { total_usages / total_integers } else { 0 };
    
    println!("📊 Summary Statistics:");
    println!("   Total unique integers: {}", total_integers);
    println!("   Total usage instances: {}", total_usages);
    println!("   Average usage per integer: {}", avg_usage);
    println!("   Report saved to: integer_usage_relations_report.json");
    
    Ok(())
}

fn analyze_node_usage(symbol: &str, node: &serde_json::Value, reports: &mut HashMap<i32, NodeUsageReport>) {
    // Extract integers from symbol name
    let integers = extract_integers_from_symbol(symbol);
    
    // Determine node characteristics
    let user_type = classify_node_type(symbol);
    let relation_type = determine_relation_type(symbol);
    let ast_context = extract_ast_context(symbol);
    let crate_name = extract_crate_name(symbol);
    
    // Get node properties
    let level = node.get("level").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let weight = node.get("weight").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let layer = node.get("layer").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    // Update reports for each integer found
    for integer in integers {
        let report = reports.entry(integer).or_insert_with(|| NodeUsageReport {
            integer_value: integer,
            total_usage_count: 0,
            user_types: HashMap::new(),
            relations: HashMap::new(),
            ast_contexts: HashMap::new(),
            crate_distribution: HashMap::new(),
        });
        
        report.total_usage_count += 1;
        *report.user_types.entry(user_type.clone()).or_insert(0) += 1;
        *report.relations.entry(relation_type.clone()).or_insert(0) += 1;
        *report.ast_contexts.entry(ast_context.clone()).or_insert(0) += 1;
        *report.crate_distribution.entry(crate_name.clone()).or_insert(0) += 1;
        
        // Add layer and level as context
        *report.ast_contexts.entry(format!("layer_{}", layer)).or_insert(0) += 1;
        *report.ast_contexts.entry(format!("level_{}", level)).or_insert(0) += 1;
    }
}

fn extract_integers_from_symbol(symbol: &str) -> Vec<i32> {
    let mut integers = Vec::new();
    let mut current_num = String::new();
    let mut is_negative = false;
    
    for (i, ch) in symbol.chars().enumerate() {
        if ch == '-' && (i == 0 || !symbol.chars().nth(i-1).unwrap_or(' ').is_ascii_digit()) {
            is_negative = true;
        } else if ch.is_ascii_digit() {
            current_num.push(ch);
        } else {
            if !current_num.is_empty() {
                if let Ok(num) = current_num.parse::<i32>() {
                    let final_num = if is_negative { -num } else { num };
                    integers.push(final_num);
                }
                current_num.clear();
                is_negative = false;
            }
        }
    }
    
    // Handle number at end
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<i32>() {
            let final_num = if is_negative { -num } else { num };
            integers.push(final_num);
        }
    }
    
    integers
}

fn classify_node_type(symbol: &str) -> String {
    if symbol.contains("::fn::") || symbol.contains("_fn_") {
        "function".to_string()
    } else if symbol.contains("::struct::") || symbol.contains("_struct_") {
        "struct".to_string()
    } else if symbol.contains("::enum::") || symbol.contains("_enum_") {
        "enum".to_string()
    } else if symbol.contains("::const::") || symbol.contains("_const_") {
        "constant".to_string()
    } else if symbol.contains("::impl::") || symbol.contains("_impl_") {
        "implementation".to_string()
    } else if symbol.contains("::trait::") || symbol.contains("_trait_") {
        "trait".to_string()
    } else if symbol.contains("::mod::") || symbol.contains("_mod_") {
        "module".to_string()
    } else if symbol.contains("wrapped_") {
        "wrapped_declaration".to_string()
    } else {
        "unknown".to_string()
    }
}

fn determine_relation_type(symbol: &str) -> String {
    if symbol.contains("module_not_found") {
        "missing_dependency".to_string()
    } else if symbol.contains("decls_") {
        "declaration_split".to_string()
    } else if symbol.contains("wrapped_") {
        "overlay_transformation".to_string()
    } else if symbol.contains("::") {
        "namespace_relation".to_string()
    } else {
        "direct_usage".to_string()
    }
}

fn extract_ast_context(symbol: &str) -> String {
    if symbol.contains("rustc_") {
        "compiler_core".to_string()
    } else if symbol.contains("std::") || symbol.contains("core::") {
        "standard_library".to_string()
    } else if symbol.contains("alloc") {
        "memory_management".to_string()
    } else if symbol.contains("hash") || symbol.contains("crypto") {
        "cryptography".to_string()
    } else if symbol.contains("io") || symbol.contains("net") {
        "input_output".to_string()
    } else if symbol.contains("test") {
        "testing".to_string()
    } else {
        "application_code".to_string()
    }
}

fn extract_crate_name(symbol: &str) -> String {
    symbol.split("::").next().unwrap_or("unknown").to_string()
}
