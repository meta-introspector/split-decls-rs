use std::collections::HashMap;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 COMMON FORMS FROM EACH COMPRESSION LAYER");
    println!("==========================================");
    
    let translation_data = fs::read_to_string("output2_emoji_translation.json")?;
    let translation: serde_json::Value = serde_json::from_str(&translation_data)?;
    
    let compressions = translation["layer_compressions"].as_array().unwrap();
    
    for (layer_idx, layer_data) in compressions.iter().enumerate() {
        let layer = layer_data["layer"].as_u64().unwrap();
        let input_patterns = layer_data["input_patterns"].as_array().unwrap();
        let output_patterns = layer_data["output_emojis"].as_array().unwrap();
        
        println!("\n## LAYER {} COMMON FORMS", layer);
        println!("Input: {} → Output: {} ({}x compression)", 
            input_patterns.len(), output_patterns.len(),
            input_patterns.len() as f64 / output_patterns.len() as f64);
        
        // Analyze input patterns
        let input_forms = analyze_common_forms(input_patterns, "INPUT");
        
        // Analyze output patterns  
        let output_forms = analyze_common_forms(output_patterns, "OUTPUT");
        
        // Show top 3 common forms for each
        println!("\n### Input Common Forms:");
        for (i, (pattern, count)) in input_forms.iter().take(3).enumerate() {
            println!("{}. **{}** ({})", i + 1, pattern, count);
        }
        
        println!("\n### Output Common Forms:");
        for (i, (pattern, count)) in output_forms.iter().take(3).enumerate() {
            println!("{}. **{}** ({})", i + 1, pattern, count);
        }
        
        // Show transformation pattern
        if !output_patterns.is_empty() {
            println!("\n### Transformation Pattern:");
            let sample_input = input_patterns[0].as_str().unwrap_or("");
            let sample_output = output_patterns[0].as_str().unwrap_or("");
            println!("Example: `{}` → `{}`", 
                truncate_string(sample_input, 50), 
                sample_output);
        }
        
        println!("\n---");
    }
    
    // Final summary
    println!("\n🎯 LAYER PROGRESSION SUMMARY:");
    println!("============================");
    
    for (layer_idx, layer_data) in compressions.iter().enumerate() {
        let layer = layer_data["layer"].as_u64().unwrap();
        let input_size = layer_data["input_patterns"].as_array().unwrap().len();
        let output_size = layer_data["output_emojis"].as_array().unwrap().len();
        let ratio = input_size as f64 / output_size as f64;
        
        println!("Layer {}: {} → {} ({:.2}x)", layer, input_size, output_size, ratio);
    }
    
    println!("\n🎪 FINAL FORM: 9 Emoji Tokens");
    println!("🦄🔮🌟🎨🎪🐉💎🎭🦋");
    
    Ok(())
}

fn analyze_common_forms(patterns: &[serde_json::Value], label: &str) -> Vec<(String, usize)> {
    let mut pattern_counts = HashMap::new();
    
    for pattern in patterns {
        let pattern_str = pattern.as_str().unwrap_or("");
        
        // Extract common form (simplified pattern)
        let common_form = extract_common_form(pattern_str);
        *pattern_counts.entry(common_form).or_insert(0) += 1;
    }
    
    let mut sorted_patterns: Vec<_> = pattern_counts.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));
    
    sorted_patterns
}

fn extract_common_form(pattern: &str) -> String {
    if pattern.chars().all(|c| c.is_ascii() && (c.is_alphabetic() || "🦄🔮🌟🎨🎪🐉💎🎭🦋".contains(c))) {
        // This is likely an emoji or simple token
        pattern.to_string()
    } else {
        // This is a file path - extract the common structure
        let parts: Vec<&str> = pattern.split('/').collect();
        
        if parts.len() >= 3 {
            // Extract pattern like "output2/*/src" or "src/*/decls"
            let first = parts.get(0).unwrap_or(&"");
            let last = parts.last().unwrap_or(&"");
            
            if parts.len() > 5 {
                format!("{}/.../{}/.../{}",
                    first,
                    parts.get(parts.len() / 2).unwrap_or(&"*"),
                    last
                )
            } else if parts.len() > 3 {
                format!("{}/.../{}", first, last)
            } else {
                format!("{}/{}/{}", parts[0], "*", parts[parts.len() - 1])
            }
        } else {
            // Short path, keep as is but generalize
            pattern.replace(char::is_numeric, "*")
        }
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
