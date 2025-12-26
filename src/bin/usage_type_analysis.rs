use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug)]
struct ConstantUsage {
    value: i32,
    count: usize,
    usage_types: Vec<String>,
    contexts: Vec<String>,
}

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut usage_analysis: HashMap<i32, ConstantUsage> = HashMap::new();
    
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, _node) in nodes {
            analyze_symbol_usage(symbol, &mut usage_analysis);
        }
    }
    
    // Sort by frequency
    let mut sorted_usage: Vec<_> = usage_analysis.into_values().collect();
    sorted_usage.sort_by(|a, b| b.count.cmp(&a.count));
    
    println!("🔍 Constant Usage Type Analysis:");
    println!("Value | Count | Primary Usage Types");
    println!("------|-------|-------------------");
    
    for usage in sorted_usage.iter().take(15) {
        let primary_types = usage.usage_types.iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        println!("{:5} | {:5} | {}", usage.value, usage.count, primary_types);
    }
    
    // Categorize by usage patterns
    let mut categories = HashMap::new();
    for usage in &sorted_usage {
        let category = categorize_usage(usage.value, &usage.contexts);
        categories.entry(category).or_insert_with(Vec::new).push(usage.value);
    }
    
    println!("\n📊 Usage Categories:");
    for (category, values) in categories {
        println!("🎯 {}: {:?}", category, values.iter().take(10).collect::<Vec<_>>());
    }
    
    Ok(())
}

fn analyze_symbol_usage(symbol: &str, usage_analysis: &mut HashMap<i32, ConstantUsage>) {
    let mut current_num = String::new();
    let mut is_negative = false;
    let chars: Vec<char> = symbol.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '-' && (i == 0 || !chars.get(i-1).unwrap_or(&' ').is_ascii_digit()) {
            is_negative = true;
        } else if ch.is_ascii_digit() {
            current_num.push(ch);
        } else {
            if !current_num.is_empty() {
                if let Ok(num) = current_num.parse::<i32>() {
                    let final_num = if is_negative { -num } else { num };
                    if final_num >= -100 && final_num <= 100 {
                        let usage_type = determine_usage_type(symbol, final_num, i);
                        let context = extract_context(symbol, i);
                        
                        let entry = usage_analysis.entry(final_num).or_insert_with(|| ConstantUsage {
                            value: final_num,
                            count: 0,
                            usage_types: Vec::new(),
                            contexts: Vec::new(),
                        });
                        
                        entry.count += 1;
                        if !entry.usage_types.contains(&usage_type) {
                            entry.usage_types.push(usage_type);
                        }
                        if !entry.contexts.contains(&context) {
                            entry.contexts.push(context);
                        }
                    }
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
            if final_num >= -100 && final_num <= 100 {
                let usage_type = determine_usage_type(symbol, final_num, chars.len());
                let context = extract_context(symbol, chars.len());
                
                let entry = usage_analysis.entry(final_num).or_insert_with(|| ConstantUsage {
                    value: final_num,
                    count: 0,
                    usage_types: Vec::new(),
                    contexts: Vec::new(),
                });
                
                entry.count += 1;
                if !entry.usage_types.contains(&usage_type) {
                    entry.usage_types.push(usage_type);
                }
                if !entry.contexts.contains(&context) {
                    entry.contexts.push(context);
                }
            }
        }
    }
}

fn determine_usage_type(symbol: &str, value: i32, _pos: usize) -> String {
    let lower = symbol.to_lowercase();
    
    if lower.contains("hash") || lower.contains("sha") || lower.contains("md5") {
        "crypto".to_string()
    } else if lower.contains("bit") || lower.contains("mask") || (value > 0 && (value & (value - 1)) == 0) {
        "bitwise".to_string()
    } else if lower.contains("size") || lower.contains("len") || lower.contains("capacity") {
        "sizing".to_string()
    } else if lower.contains("index") || lower.contains("offset") || lower.contains("pos") {
        "indexing".to_string()
    } else if lower.contains("version") || lower.contains("major") || lower.contains("minor") {
        "versioning".to_string()
    } else if lower.contains("error") || lower.contains("code") || value < 0 {
        "error_code".to_string()
    } else if lower.contains("test") || lower.contains("mock") || lower.contains("example") {
        "testing".to_string()
    } else if lower.contains("const") || lower.contains("static") {
        "constant".to_string()
    } else {
        "general".to_string()
    }
}

fn extract_context(symbol: &str, _pos: usize) -> String {
    if let Some(first_part) = symbol.split("::").next() {
        first_part.to_string()
    } else {
        "unknown".to_string()
    }
}

fn categorize_usage(value: i32, contexts: &[String]) -> String {
    match value {
        0 => "null_values",
        1 => "identity_values", 
        -1 => "negation_values",
        2 | 4 | 8 | 16 | 32 | 64 => "power_of_two",
        -2 | -4 | -8 | -16 | -32 | -64 => "negative_power_of_two",
        3 | 5 | 6 | 7 => "small_primes_composites",
        _ if value > 0 && value <= 10 => "small_positive",
        _ if value < 0 && value >= -10 => "small_negative", 
        _ if value > 10 && value <= 100 => "medium_positive",
        _ if value < -10 && value >= -100 => "medium_negative",
        _ => "other"
    }.to_string()
}
