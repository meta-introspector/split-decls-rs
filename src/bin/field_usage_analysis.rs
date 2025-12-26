use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FieldUsageReport {
    integer_value: i32,
    total_usage_count: usize,
    field_usage: HashMap<String, usize>, // field name -> count
    field_types: HashMap<String, usize>, // field type -> count
    usage_contexts: HashMap<String, usize>, // where it's used -> count
}

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut field_reports: HashMap<i32, FieldUsageReport> = HashMap::new();
    
    // Analyze field usage for each node
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, node) in nodes {
            analyze_field_usage(symbol, node, &mut field_reports);
        }
    }
    
    // Sort by usage count
    let mut sorted_reports: Vec<_> = field_reports.into_values().collect();
    sorted_reports.sort_by(|a, b| b.total_usage_count.cmp(&a.total_usage_count));
    
    println!("📊 Integer Field Usage Analysis");
    println!("===============================\n");
    
    for (i, report) in sorted_reports.iter().take(15).enumerate() {
        println!("{}. Integer: {} (Used in {} fields)", i+1, report.integer_value, report.total_usage_count);
        
        // Top fields using this integer
        let mut fields: Vec<_> = report.field_usage.iter().collect();
        fields.sort_by(|a, b| b.1.cmp(a.1));
        println!("   🏷️  Fields: {}", 
            fields.iter().take(5)
                .map(|(field, count)| format!("{}({})", field, count))
                .collect::<Vec<_>>().join(", "));
        
        // Field types
        let mut types: Vec<_> = report.field_types.iter().collect();
        types.sort_by(|a, b| b.1.cmp(a.1));
        println!("   📝 Types: {}", 
            types.iter().take(3)
                .map(|(t, c)| format!("{}({})", t, c))
                .collect::<Vec<_>>().join(", "));
        
        // Usage contexts
        let mut contexts: Vec<_> = report.usage_contexts.iter().collect();
        contexts.sort_by(|a, b| b.1.cmp(a.1));
        println!("   🎯 Contexts: {}", 
            contexts.iter().take(3)
                .map(|(ctx, c)| format!("{}({})", ctx, c))
                .collect::<Vec<_>>().join(", "));
        println!();
    }
    
    // Save detailed report
    fs::write("field_usage_report.json", serde_json::to_string_pretty(&sorted_reports)?)?;
    println!("📄 Detailed report saved to: field_usage_report.json");
    
    Ok(())
}

fn analyze_field_usage(symbol: &str, node: &serde_json::Value, reports: &mut HashMap<i32, FieldUsageReport>) {
    // Extract integers from symbol
    let integers = extract_integers_from_symbol(symbol);
    
    // Analyze node fields that contain integers
    if let Some(level) = node.get("level").and_then(|v| v.as_u64()) {
        let level_int = level as i32;
        update_field_report(level_int, "level", "u32", "node_metadata", reports);
    }
    
    if let Some(weight) = node.get("weight").and_then(|v| v.as_f64()) {
        let weight_int = (weight * 100.0) as i32;
        if weight_int >= -1000 && weight_int <= 1000 {
            update_field_report(weight_int, "weight", "f64", "node_metadata", reports);
        }
    }
    
    if let Some(complexity) = node.get("complexity_score").and_then(|v| v.as_f64()) {
        let complexity_int = complexity as i32;
        if complexity_int >= -100 && complexity_int <= 1000 {
            update_field_report(complexity_int, "complexity_score", "f64", "node_metadata", reports);
        }
    }
    
    // Analyze integers found in symbol names
    for integer in integers {
        let field_context = determine_field_context(symbol, integer);
        let field_type = infer_field_type(symbol, integer);
        let usage_context = extract_usage_context(symbol);
        
        update_field_report(integer, &field_context, &field_type, &usage_context, reports);
    }
}

fn update_field_report(
    integer: i32, 
    field_name: &str, 
    field_type: &str, 
    context: &str, 
    reports: &mut HashMap<i32, FieldUsageReport>
) {
    let report = reports.entry(integer).or_insert_with(|| FieldUsageReport {
        integer_value: integer,
        total_usage_count: 0,
        field_usage: HashMap::new(),
        field_types: HashMap::new(),
        usage_contexts: HashMap::new(),
    });
    
    report.total_usage_count += 1;
    *report.field_usage.entry(field_name.to_string()).or_insert(0) += 1;
    *report.field_types.entry(field_type.to_string()).or_insert(0) += 1;
    *report.usage_contexts.entry(context.to_string()).or_insert(0) += 1;
}

fn determine_field_context(symbol: &str, integer: i32) -> String {
    let symbol_lower = symbol.to_lowercase();
    
    if symbol_lower.contains("version") {
        format!("version_field_{}", integer)
    } else if symbol_lower.contains("size") || symbol_lower.contains("len") {
        format!("size_field_{}", integer)
    } else if symbol_lower.contains("index") || symbol_lower.contains("offset") {
        format!("index_field_{}", integer)
    } else if symbol_lower.contains("hash") || symbol_lower.contains("sha") {
        format!("hash_field_{}", integer)
    } else if symbol_lower.contains("bit") || symbol_lower.contains("mask") {
        format!("bitfield_{}", integer)
    } else if symbol_lower.contains("error") || symbol_lower.contains("code") {
        format!("error_field_{}", integer)
    } else if symbol_lower.contains("id") || symbol_lower.contains("identifier") {
        format!("id_field_{}", integer)
    } else if symbol_lower.contains("count") || symbol_lower.contains("num") {
        format!("counter_field_{}", integer)
    } else {
        format!("generic_field_{}", integer)
    }
}

fn infer_field_type(symbol: &str, integer: i32) -> String {
    if integer == 0 || integer == 1 || integer == -1 {
        "flag_or_boolean"
    } else if integer > 0 && (integer & (integer - 1)) == 0 {
        "power_of_two"
    } else if integer < 0 {
        "error_code_or_offset"
    } else if integer > 1000 {
        "large_constant"
    } else if integer > 100 {
        "medium_constant"
    } else {
        "small_integer"
    }.to_string()
}

fn extract_usage_context(symbol: &str) -> String {
    if symbol.contains("rustc_") {
        "compiler_internals"
    } else if symbol.contains("std::") || symbol.contains("core::") {
        "standard_library"
    } else if symbol.contains("crypto") || symbol.contains("hash") {
        "cryptography"
    } else if symbol.contains("net") || symbol.contains("io") {
        "networking_io"
    } else if symbol.contains("test") {
        "testing_framework"
    } else if symbol.contains("wrapped_") {
        "overlay_system"
    } else {
        "application_code"
    }.to_string()
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
