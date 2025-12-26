use std::collections::HashMap;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut constant_counts: HashMap<i32, usize> = HashMap::new();
    
    // Count occurrences of each numeric constant in source code
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, _node) in nodes {
            extract_and_count_numerics(symbol, &mut constant_counts);
        }
    }
    
    // Sort by value and display counts
    let mut sorted_constants: Vec<(i32, usize)> = constant_counts.into_iter().collect();
    sorted_constants.sort_by_key(|&(value, _)| value);
    
    println!("🔢 Constant Frequency Analysis (-100 to 100):");
    println!("Value | Count | Frequency");
    println!("------|-------|----------");
    
    for value in -100..=100 {
        let count = sorted_constants.iter()
            .find(|(v, _)| *v == value)
            .map(|(_, c)| *c)
            .unwrap_or(0);
        
        if count > 0 {
            let freq = "*".repeat((count / 5).max(1).min(20));
            println!("{:5} | {:5} | {}", value, count, freq);
        }
    }
    
    // Summary statistics
    let total_occurrences: usize = sorted_constants.iter().map(|(_, count)| count).sum();
    let unique_values = sorted_constants.len();
    let max_count = sorted_constants.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let most_frequent = sorted_constants.iter().max_by_key(|(_, count)| *count);
    
    println!("\n📊 Summary:");
    println!("   Total occurrences: {}", total_occurrences);
    println!("   Unique values found: {}", unique_values);
    println!("   Max frequency: {}", max_count);
    if let Some((value, count)) = most_frequent {
        println!("   Most frequent: {} (appears {} times)", value, count);
    }
    
    Ok(())
}

fn extract_and_count_numerics(symbol: &str, counts: &mut HashMap<i32, usize>) {
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
                    if final_num >= -100 && final_num <= 100 {
                        *counts.entry(final_num).or_insert(0) += 1;
                    }
                }
                current_num.clear();
                is_negative = false;
            }
        }
    }
    
    // Handle number at end of string
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<i32>() {
            let final_num = if is_negative { -num } else { num };
            if final_num >= -100 && final_num <= 100 {
                *counts.entry(final_num).or_insert(0) += 1;
            }
        }
    }
}
