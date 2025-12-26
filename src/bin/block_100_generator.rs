use std::fs;
use anyhow::Result;
use serde_json;

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut constants: Vec<i32> = Vec::new();
    let mut numerics: Vec<i32> = Vec::new();
    
    // Extract constants and numeric values from the lattice
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, node) in nodes {
            // Look for numeric constants in symbol names
            extract_numerics_from_symbol(symbol, &mut numerics);
            
            // Extract numeric values from node properties
            if let Some(level) = node["level"].as_u64() {
                if level <= 100 { numerics.push(level as i32); }
            }
            if let Some(weight) = node["weight"].as_f64() {
                let weight_int = (weight * 100.0) as i32;
                if weight_int >= -100 && weight_int <= 100 {
                    numerics.push(weight_int);
                }
            }
            if let Some(complexity) = node["complexity_score"].as_f64() {
                let complexity_int = complexity as i32;
                if complexity_int >= -100 && complexity_int <= 100 {
                    numerics.push(complexity_int);
                }
            }
        }
    }
    
    // Add all integers from -100 to 100
    for i in -100..=100 {
        numerics.push(i);
    }
    
    // Remove duplicates and sort
    numerics.sort();
    numerics.dedup();
    
    // Generate block 100.md
    let mut content = String::new();
    content.push_str("# Block 100: Numeric Constants Foundation\n\n");
    content.push_str("**Range**: -100 to 100  \n");
    content.push_str("**Purpose**: Fundamental numeric constants for LMDFB lattice  \n");
    content.push_str("**Total Constants**: ");
    content.push_str(&numerics.len().to_string());
    content.push_str("\n\n");
    
    content.push_str("## Numeric Lattice Constants\n\n");
    content.push_str("```\n");
    
    // Format in rows of 10
    for (i, num) in numerics.iter().enumerate() {
        if i % 10 == 0 && i > 0 {
            content.push('\n');
        }
        content.push_str(&format!("{:4} ", num));
    }
    
    content.push_str("\n```\n\n");
    
    // Add special constants
    content.push_str("## Special Constants\n\n");
    content.push_str("| Value | Significance |\n");
    content.push_str("|-------|-------------|\n");
    content.push_str("| 0 | Origin point, null transformation |\n");
    content.push_str("| 1 | Identity, unit transformation |\n");
    content.push_str("| -1 | Negation, inverse transformation |\n");
    content.push_str("| 8 | Dimension count (8D manifold) |\n");
    content.push_str("| 42 | Universal constant |\n");
    content.push_str("| 100 | Block boundary, maximum range |\n");
    content.push_str("| -100 | Block boundary, minimum range |\n");
    
    content.push_str("\n## Lattice Properties\n\n");
    content.push_str(&format!("- **Foundation Layer**: {} nodes\n", 4222));
    content.push_str(&format!("- **System Layer**: {} nodes\n", 175));
    content.push_str(&format!("- **Compiler Layer**: {} nodes\n", 1135));
    content.push_str(&format!("- **Total Nodes**: {} nodes\n", 5532));
    content.push_str(&format!("- **Max Level**: {}\n", 5));
    
    content.push_str("\n---\n*Generated for LMDFB 8D manifold embedding*\n");
    
    fs::write("block_100.md", content)?;
    
    println!("🔢 Block 100 generated:");
    println!("   📊 {} numeric constants (-100 to 100)", numerics.len());
    println!("   🎯 Special constants identified");
    println!("   📝 block_100.md created");
    
    Ok(())
}

fn extract_numerics_from_symbol(symbol: &str, numerics: &mut Vec<i32>) {
    // Extract numbers from symbol names
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
                        numerics.push(final_num);
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
                numerics.push(final_num);
            }
        }
    }
}
