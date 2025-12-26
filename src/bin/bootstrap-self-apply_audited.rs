// Comprehensive syscall audit macros for bootstrap
use std::time::{SystemTime, Instant};

macro_rules! audit_execute {
    ($cmd:expr) => {{
        let start = Instant::now();
        let timestamp = SystemTime::now();
        println!("⚠️  PROCESS AUDIT: {:?}", timestamp);
        println!("📋 Command: {}", stringify!($cmd));
        println!("📁 PWD: {:?}", std::env::current_dir().unwrap_or_default());
        
        let result = $cmd;
        let duration = start.elapsed();
        
        match &result {
            Ok(output) => {
                println!("✅ Process completed in {:?}", duration);
                if let Some(code) = output.status.code() {
                    println!("📤 Exit: {}", code);
                }
            }
            Err(e) => println!("❌ Process failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_write {
    ($path:expr, $contents:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  FILE WRITE AUDIT: {:?}", timestamp);
        println!("📝 Writing to: {:?}", $path);
        println!("📊 Size: {} bytes", $contents.len());
        
        let result = std::fs::write($path, $contents);
        
        match &result {
            Ok(_) => println!("✅ File written successfully"),
            Err(e) => println!("❌ File write failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_create_dir_all {
    ($path:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  DIR CREATE AUDIT: {:?}", timestamp);
        println!("📁 Creating: {:?}", $path);
        
        let result = std::fs::create_dir_all($path);
        
        match &result {
            Ok(_) => println!("✅ Directory created successfully"),
            Err(e) => println!("❌ Directory creation failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

use std::fs;
use split_decls_rs::macro_interpreter::RdfStateMachine;
use split_decls_rs::ast_statistics::AstStatistics;
use split_decls_rs::{interpret_wrapped_decl, interpret_syn_function};

fn main() -> anyhow::Result<()> {
    println!("🚀 SELF-APPLYING BOOTSTRAP SYSTEM");
    println!("Using wrapped syn macros to analyze and compress code into 8D type manifolds\n");

    let mut rdf_state = RdfStateMachine::new();
    let mut ast_stats = AstStatistics::new();

    // Start by analyzing our own macro interpreter code
    println!("📊 ANALYZING MACRO INTERPRETER CODE:");
    let macro_interpreter_code = fs::read_to_string("src/macro_interpreter.rs")?;
    ast_stats.analyze_with_wrapped_syn(&macro_interpreter_code, &mut rdf_state)?;

    // Analyze the AST statistics code (self-analysis)
    println!("🔍 SELF-ANALYZING AST STATISTICS CODE:");
    let ast_stats_code = fs::read_to_string("src/ast_statistics.rs")?;
    ast_stats.analyze_with_wrapped_syn(&ast_stats_code, &mut rdf_state)?;

    // Analyze some of the wrapped syn declarations
    println!("🎯 ANALYZING WRAPPED SYN DECLARATIONS:");
    let wrapped_files = [
        "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse.rs",
        "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_token.rs",
        "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_expr.rs",
    ];

    for file_path in &wrapped_files {
        if let Ok(code) = fs::read_to_string(file_path) {
            println!("  Analyzing: {}", file_path);
            ast_stats.analyze_with_wrapped_syn(&code, &mut rdf_state)?;
        }
    }

    // Emit comprehensive statistics
    println!("\n📈 GENERATING 8D TYPE MANIFOLD STATISTICS:");
    ast_stats.emit_rdf_statistics(&mut rdf_state);

    // Output results
    println!("\n🔗 BOOTSTRAP RDF TRACE:");
    println!("Total RDF triples: {}", rdf_state.triples.len());
    
    // Show sample of statistical RDF triples
    let stat_triples: Vec<_> = rdf_state.triples.iter()
        .filter(|t| t.predicate.contains("manifold") || t.predicate.contains("count") || t.predicate.contains("ast_type"))
        .take(10)
        .collect();
    
    for triple in stat_triples {
        println!("  {} -> {} -> {}", triple.subject, triple.predicate, triple.object);
    }

    // Save compressed representation
    println!("\n💾 SAVING COMPRESSED AST REPRESENTATION:");
    let compressed_data = serde_json::to_string_pretty(&ast_stats)?;
    audit_fs_write!("bootstrap_ast_compression.json", compressed_data)?;
    println!("  Saved to: bootstrap_ast_compression.json");

    // Save RDF graph
    let rdf_ttl = generate_rdf_ttl(&rdf_state);
    audit_fs_write!("bootstrap_self_analysis.ttl", rdf_ttl)?;
    println!("  Saved RDF graph to: bootstrap_self_analysis.ttl");

    println!("\n✅ SELF-APPLYING BOOTSTRAP COMPLETE");
    println!("🎯 Ready for recursive generation using compressed AST patterns");

    Ok(())
}

fn generate_rdf_ttl(rdf_state: &RdfStateMachine) -> String {
    let mut ttl = String::from("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    ttl.push_str("@prefix ast: <http://split-decls.rs/ast#> .\n");
    ttl.push_str("@prefix exec: <http://split-decls.rs/execution#> .\n\n");

    for triple in &rdf_state.triples {
        ttl.push_str(&format!(
            "ast:{} ast:{} \"{}\" .\n",
            triple.subject.replace("::", "_"),
            triple.predicate.replace(":", "_"),
            triple.object
        ));
    }

    ttl
}
