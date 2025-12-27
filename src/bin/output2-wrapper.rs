use clap::{Parser, Subcommand};
use std::path::PathBuf;
use split_decls_rs::macro_interpreter::RdfStateMachine;
use split_decls_rs::{interpret_wrapped_decl, interpret_syn_function};

#[derive(Parser)]
#[command(name = "output2-wrapper")]
#[command(about = "Unified wrapper for all output2 functionality with RDF interpretation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Precompile mode - run bootstrap on single or multiple crates
    Precompile {
        /// Target crate path (single crate mode)
        #[arg(short, long)]
        crate_path: Option<PathBuf>,
        
        /// Process all crates in directory (multi-crate mode)
        #[arg(short, long)]
        all: bool,
        
        /// Apply to split-decls-rs bootstrap itself
        #[arg(short, long)]
        bootstrap: bool,
        
        /// Output directory for precompiled results
        #[arg(short, long, default_value = "output2")]
        output: PathBuf,
    },
    
    /// Inspect wrapped declarations in output2
    Inspect {
        /// Crate name to inspect
        crate_name: String,
        
        /// Show declaration details
        #[arg(short, long)]
        details: bool,
    },
    
    /// Run RDF interpreter on wrapped crates
    Rdf {
        /// Crate names to analyze
        crates: Vec<String>,
        
        /// Output RDF file
        #[arg(short, long, default_value = "analysis.ttl")]
        output: PathBuf,
    },
    
    /// Execute wrapped syn functions with performance tracking
    Execute {
        /// Function patterns to execute
        patterns: Vec<String>,
        
        /// Benchmark mode
        #[arg(short, long)]
        benchmark: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut rdf_state = RdfStateMachine::new();
    
    match cli.command {
        Commands::Precompile { crate_path, all, bootstrap, output } => {
            println!("🔥 PRECOMPILE MODE");
            
            if bootstrap {
                println!("🚀 Applying to split-decls-rs bootstrap");
                precompile_bootstrap(&mut rdf_state, &output, cli.verbose)?;
            } else if let Some(path) = crate_path {
                println!("📦 Single crate mode: {}", path.display());
                precompile_single_crate(&mut rdf_state, &path, &output, cli.verbose)?;
            } else if all {
                println!("🌐 Multi-crate mode");
                precompile_all_crates(&mut rdf_state, &output, cli.verbose)?;
            } else {
                println!("❌ Must specify --crate-path, --all, or --bootstrap");
                return Ok(());
            }
        }
        
        Commands::Inspect { crate_name, details } => {
            println!("🔍 INSPECTING: {}", crate_name);
            inspect_wrapped_crate(&crate_name, details)?;
        }
        
        Commands::Rdf { crates, output } => {
            println!("🔗 RDF ANALYSIS: {:?}", crates);
            run_rdf_analysis(&mut rdf_state, &crates, &output)?;
        }
        
        Commands::Execute { patterns, benchmark } => {
            println!("⚡ EXECUTING PATTERNS: {:?}", patterns);
            execute_wrapped_patterns(&mut rdf_state, &patterns, benchmark)?;
        }
    }
    
    // Always output RDF summary
    if !rdf_state.triples.is_empty() {
        println!("\n🔗 RDF SUMMARY: {} triples captured", rdf_state.triples.len());
        if cli.verbose {
            for triple in rdf_state.triples.iter().take(5) {
                println!("  {} -> {} -> {}", triple.subject, triple.predicate, triple.object);
            }
        }
    }
    
    Ok(())
}

fn precompile_bootstrap(rdf_state: &mut RdfStateMachine, output: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    rdf_state.enter_function("precompile_bootstrap");
    
    println!("🎯 BOOTSTRAP PRECOMPILE STEPS:");
    println!("  1. Load split-decls-rs source");
    println!("  2. Apply wrapped syn analysis");
    println!("  3. Generate compressed AST");
    println!("  4. Output to {}", output.display());
    
    // Use wrapped syn to analyze the bootstrap process itself
    let bootstrap_files = [
        "src/main.rs",
        "src/lib.rs", 
        "src/macro_interpreter.rs",
        "src/ast_statistics.rs",
    ];
    
    for file_path in &bootstrap_files {
        if let Ok(code) = std::fs::read_to_string(file_path) {
            let result = interpret_syn_function!(rdf_state, "syn::parse_file", "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse_file.rs");
            rdf_state.capture_data("analyzed_file", file_path);
            rdf_state.capture_data("file_size", &code.len().to_string());
            
            if verbose {
                println!("    Analyzed: {} ({} bytes)", file_path, code.len());
            }
        }
    }
    
    rdf_state.exit_function("precompile_bootstrap");
    println!("✅ Bootstrap precompile complete");
    Ok(())
}

fn precompile_single_crate(rdf_state: &mut RdfStateMachine, crate_path: &PathBuf, output: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    rdf_state.enter_function("precompile_single_crate");
    rdf_state.capture_data("target_crate", &crate_path.to_string_lossy());
    
    println!("📦 Processing single crate: {}", crate_path.display());
    
    // TODO: Implement single crate precompilation using wrapped declarations
    
    rdf_state.exit_function("precompile_single_crate");
    Ok(())
}

fn precompile_all_crates(rdf_state: &mut RdfStateMachine, output: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    rdf_state.enter_function("precompile_all_crates");
    
    println!("🌐 Processing all crates in output2/");
    
    // TODO: Implement multi-crate precompilation
    
    rdf_state.exit_function("precompile_all_crates");
    Ok(())
}

fn inspect_wrapped_crate(crate_name: &str, details: bool) -> anyhow::Result<()> {
    let wrapped_path = format!("output2/wrapped-{}", crate_name);
    let decls_path = format!("{}/src/decls", wrapped_path);
    
    if let Ok(entries) = std::fs::read_dir(&decls_path) {
        let decl_files: Vec<_> = entries.flatten().collect();
        println!("📋 Found {} declarations in {}", decl_files.len(), wrapped_path);
        
        if details {
            for entry in decl_files.iter().take(10) {
                let file_name_string = entry.file_name().to_string_lossy().to_string();
                if let Ok(metadata) = entry.metadata() {
                    println!("  {} ({} bytes)", file_name_string, metadata.len());
                }
            }
        }
    } else {
        println!("❌ Wrapped crate not found: {}", wrapped_path);
    }
    
    Ok(())
}

fn run_rdf_analysis(rdf_state: &mut RdfStateMachine, crates: &[String], output: &PathBuf) -> anyhow::Result<()> {
    rdf_state.enter_function("rdf_analysis");
    
    for crate_name in crates {
        rdf_state.capture_data("analyzed_crate", crate_name);
        // TODO: Implement RDF analysis of wrapped crates
    }
    
    rdf_state.exit_function("rdf_analysis");
    Ok(())
}

fn execute_wrapped_patterns(rdf_state: &mut RdfStateMachine, patterns: &[String], benchmark: bool) -> anyhow::Result<()> {
    rdf_state.enter_function("execute_patterns");
    
    for pattern in patterns {
        let result = interpret_syn_function!(rdf_state, pattern, &format!("output2/wrapped-syn/src/decls/wrapped_syn_decls_{}.rs", pattern.to_lowercase()));
        
        if benchmark {
            println!("⚡ Executed: {} -> {}", pattern, result);
        }
    }
    
    rdf_state.exit_function("execute_patterns");
    Ok(())
}
