use anyhow::Result;
use clap::{Arg, Command};
use std::fs;
use crate::monster_compressor::{MonsterCompressor, MonsterSignature};

fn main() -> Result<()> {
    let matches = Command::new("monster-compress")
        .version("0.1.0")
        .about("Monster Group order compression: 2^46 × 3^20 × 5^9 × 7^6")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("DIR")
            .help("Input directory with signatures")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output Monster Group mappings"))
        .get_matches();

    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");

    println!("👹 Monster Group Compression");
    println!("Order: 808,017,424,794,512,875,886,459,904,961,710,757,005,754,368,000,000,000");
    println!("Factorization: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × ...");

    let mut compressor = MonsterCompressor::new();
    let mut signatures = Vec::new();

    // Process all signature files
    let paths = fs::read_dir(input_dir)?;
    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path)?;
            
            // Extract signature from file (simplified)
            let signature = extract_signature_from_content(&content);
            if !signature.is_empty() {
                let monster_sig = compressor.compress_signature(&signature)?;
                signatures.push(monster_sig);
            }
        }
    }

    println!("\n📊 Collected {} signatures", signatures.len());

    // Apply Monster Group compression
    compressor.apply_monster_compression();

    // Display results
    display_monster_compression(&compressor, &signatures);

    // Generate output
    if let Some(output_file) = output_file {
        generate_monster_mappings(&compressor, output_file)?;
    }

    Ok(())
}

fn extract_signature_from_content(content: &str) -> String {
    let mut parts = Vec::new();
    
    if content.contains("prelude!") { parts.push("prelude"); }
    if content.contains("#[decl_") { parts.push("decl_attr"); }
    if content.contains("use ") { parts.push("use_stmt"); }
    if content.contains("pub fn") { parts.push("pub_fn"); }
    if content.contains("pub struct") { parts.push("pub_struct"); }
    if content.contains("pub enum") { parts.push("pub_enum"); }
    if content.contains("impl ") { parts.push("impl_block"); }
    if content.contains("trait ") { parts.push("trait_def"); }
    if content.contains("macro_rules!") { parts.push("macro_def"); }
    if content.contains("derive(") { parts.push("derive_attr"); }
    
    parts.join("|")
}

fn display_monster_compression(compressor: &MonsterCompressor, signatures: &[MonsterSignature]) {
    println!("\n👹 Monster Group Compression Results:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    println!("\n🔥 2^46 - Top Pairs (most common):");
    for (pair, emoji) in compressor.pair_emojis.iter().take(10) {
        let freq = compressor.pair_frequencies.get(pair).unwrap_or(&0);
        println!("  {} {} (freq: {})", emoji, pair, freq);
    }
    
    println!("\n🔺 3^20 - Top Triples:");
    for (triple, emoji) in compressor.triple_emojis.iter().take(10) {
        let freq = compressor.triple_frequencies.get(triple).unwrap_or(&0);
        println!("  {} {} (freq: {})", emoji, triple, freq);
    }
    
    println!("\n🌀 5^9 - Top 5-grams:");
    for (penta, emoji) in compressor.penta_emojis.iter().take(9) {
        let freq = compressor.penta_frequencies.get(penta).unwrap_or(&0);
        println!("  {} {} (freq: {})", emoji, penta, freq);
    }
    
    println!("\n👑 7^6 - Top 7-grams:");
    for (hepta, emoji) in compressor.hepta_emojis.iter().take(6) {
        let freq = compressor.hepta_frequencies.get(hepta).unwrap_or(&0);
        println!("  {} {} (freq: {})", emoji, hepta, freq);
    }
    
    // Calculate total compression
    let total_pairs = compressor.pair_frequencies.len();
    let total_triples = compressor.triple_frequencies.len();
    let total_pentas = compressor.penta_frequencies.len();
    let total_heptas = compressor.hepta_frequencies.len();
    
    println!("\n📈 Compression Statistics:");
    println!("  Pairs: {} total, 46 compressed ({}%)", 
        total_pairs, if total_pairs > 0 { 46 * 100 / total_pairs } else { 0 });
    println!("  Triples: {} total, 20 compressed ({}%)", 
        total_triples, if total_triples > 0 { 20 * 100 / total_triples } else { 0 });
    println!("  5-grams: {} total, 9 compressed ({}%)", 
        total_pentas, if total_pentas > 0 { 9 * 100 / total_pentas } else { 0 });
    println!("  7-grams: {} total, 6 compressed ({}%)", 
        total_heptas, if total_heptas > 0 { 6 * 100 / total_heptas } else { 0 });
    
    // Show Monster Group factorization for sample signatures
    println!("\n🧮 Sample Monster Factorizations:");
    for (i, sig) in signatures.iter().take(5).enumerate() {
        println!("  {}. {} → {}", i + 1, sig.original, sig.monster_factorization());
    }
}

fn generate_monster_mappings(compressor: &MonsterCompressor, output_file: &str) -> Result<()> {
    let mut output = String::new();
    
    output.push_str("# Monster Group Order Compression Mappings\n");
    output.push_str("# Order: 808,017,424,794,512,875,886,459,904,961,710,757,005,754,368,000,000,000\n");
    output.push_str("# Factorization: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71\n\n");
    
    output.push_str("## 2^46 - Pair Mappings (46 most common)\n");
    for (pair, emoji) in &compressor.pair_emojis {
        let freq = compressor.pair_frequencies.get(pair).unwrap_or(&0);
        output.push_str(&format!("{} {} (freq: {})\n", emoji, pair, freq));
    }
    
    output.push_str("\n## 3^20 - Triple Mappings (20 samples)\n");
    for (triple, emoji) in &compressor.triple_emojis {
        let freq = compressor.triple_frequencies.get(triple).unwrap_or(&0);
        output.push_str(&format!("{} {} (freq: {})\n", emoji, triple, freq));
    }
    
    output.push_str("\n## 5^9 - 5-gram Mappings (9 samples)\n");
    for (penta, emoji) in &compressor.penta_emojis {
        let freq = compressor.penta_frequencies.get(penta).unwrap_or(&0);
        output.push_str(&format!("{} {} (freq: {})\n", emoji, penta, freq));
    }
    
    output.push_str("\n## 7^6 - 7-gram Mappings (6 samples)\n");
    for (hepta, emoji) in &compressor.hepta_emojis {
        let freq = compressor.hepta_frequencies.get(hepta).unwrap_or(&0);
        output.push_str(&format!("{} {} (freq: {})\n", emoji, hepta, freq));
    }
    
    fs::write(output_file, output)?;
    println!("👹 Monster Group mappings written to {}", output_file);
    Ok(())
}
