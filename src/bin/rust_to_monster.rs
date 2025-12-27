use anyhow::Result;
use clap::{Arg, Command};
use std::fs;
use split_decls_rs::monster_compressor::MonsterCompressor;
use split_decls_rs::rust_to_monster_reporter::RustToMonsterReporter;

fn main() -> Result<()> {
    let matches = #[syscall="exec"]
    Command::new("rust-to-monster")
        .version("0.1.0")
        .about("Generate the epic Rust to Monster Group transformation report")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("DIR")
            .help("Input directory with Rust declarations")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file for the epic report")
            .default_value("RUST_TO_MONSTER_REPORT.md"))
        .get_matches();

    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    println!("👹 Generating EPIC Rust to Monster Group Transformation Report...");
    println!("📂 Analyzing: {}", input_dir);

    // Create and populate Monster compressor
    let mut compressor = MonsterCompressor::new();
    let mut processed_files = 0;

    // Process all .rs files
    let paths = fs::read_dir(input_dir)?;
    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path)?;
            let signature = extract_signature_from_content(&content);
            
            if !signature.is_empty() {
                match compressor.compress_signature(&signature) {
                    Ok(_) => {
                        processed_files += 1;
                        if processed_files % 100 == 0 {
                            println!("📊 Processed {} files...", processed_files);
                        }
                    }
                    Err(e) => eprintln!("⚠️ Error processing {}: {}", path.display(), e),
                }
            }
        }
    }

    println!("✅ Processed {} files total", processed_files);
    println!("🔄 Applying Monster Group compression...");

    // Apply Monster Group compression
    compressor.apply_monster_compression();

    println!("📝 Generating epic report...");

    // Create reporter and generate report
    let reporter = RustToMonsterReporter::new(compressor);
    let epic_report = reporter.generate_epic_report()?;

    // Write the epic report
    fs::write(output_file, epic_report)?;

    println!("🎉 EPIC SUCCESS!");
    println!("👹 Rust to Monster Group Transformation Report written to: {}", output_file);
    println!("📊 Total signatures analyzed: {}", reporter.total_signatures);
    println!("🔢 Total declarations processed: {}", reporter.total_declarations);
    println!("🎯 Monster Group coverage: {:.1}%", reporter.monster_coverage * 100.0);
    
    // Show preview of singletons found
    let singleton_count = reporter.compressor.singleton_17.is_some() as u8 +
                         reporter.compressor.singleton_19.is_some() as u8 +
                         reporter.compressor.singleton_23.is_some() as u8 +
                         reporter.compressor.singleton_29.is_some() as u8 +
                         reporter.compressor.singleton_31.is_some() as u8 +
                         reporter.compressor.singleton_41.is_some() as u8 +
                         reporter.compressor.singleton_47.is_some() as u8 +
                         reporter.compressor.singleton_59.is_some() as u8 +
                         reporter.compressor.singleton_71.is_some() as u8;
    
    println!("🏆 Singleton rarities discovered: {}/9", singleton_count);
    
    if let Some(ultimate) = &reporter.compressor.singleton_71 {
        println!("👹 ULTIMATE RARITY (71): {}", ultimate);
    }

    println!("\n🎭 The transformation is complete. Rust has become one with the Monster Group.");
    println!("📖 Read {} for the full epic tale!", output_file);

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
    if content.contains("async fn") { parts.push("async_fn"); }
    if content.contains("const ") { parts.push("const_item"); }
    if content.contains("static ") { parts.push("static_item"); }
    if content.contains("type ") { parts.push("type_alias"); }
    if content.contains("mod ") { parts.push("module"); }
    
    parts.sort();
    parts.dedup();
    parts.join("|")
}
