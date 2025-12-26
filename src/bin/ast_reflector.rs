use std::path::Path;
use anyhow::Result;
use clap::{Parser, Subcommand};
use split_decls_rs::ast_reflector::{AstReflector, create_example_probes, AstProbe};

#[derive(Parser)]
#[command(name = "ast-reflector")]
#[command(about = "AST Reflector - eBPF-like probes for Rust AST transformation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reflect over AST in a directory
    Reflect {
        /// Directory to reflect over
        #[arg(short, long)]
        dir: String,
        
        /// Probe configuration file
        #[arg(short, long)]
        probes: Option<String>,
        
        /// Output directory for transformed files
        #[arg(short, long)]
        output: Option<String>,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Generate example probe configuration
    GenProbes {
        /// Output file for probe configuration
        #[arg(short, long, default_value = "ast_probes.json")]
        output: String,
    },
    
    /// List available probe types and actions
    ListTypes,
    
    /// Analyze AST patterns in directory
    Analyze {
        /// Directory to analyze
        #[arg(short, long)]
        dir: String,
        
        /// Generate probes based on analysis
        #[arg(short, long)]
        generate: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Reflect { dir, probes, output, verbose } => {
            reflect_directory(&dir, probes.as_deref(), output.as_deref(), verbose)
        }
        
        Commands::GenProbes { output } => {
            generate_example_probes(&output)
        }
        
        Commands::ListTypes => {
            list_probe_types();
            Ok(())
        }
        
        Commands::Analyze { dir, generate } => {
            analyze_directory(&dir, generate)
        }
    }
}

fn reflect_directory(dir: &str, probes_file: Option<&str>, _output: Option<&str>, verbose: bool) -> Result<()> {
    let mut reflector = AstReflector::new();
    
    if verbose {
        println!("🔍 Starting AST reflection on directory: {}", dir);
    }
    
    // Load probes
    if let Some(probes_path) = probes_file {
        reflector.load_probes(Path::new(probes_path))?;
    } else {
        println!("📋 Using default example probes");
        for probe in create_example_probes() {
            reflector.add_probe(probe);
        }
    }
    
    // Reflect over the directory
    reflector.reflect_directory(Path::new(dir))?;
    
    println!("✅ AST reflection completed!");
    Ok(())
}

fn generate_example_probes(output: &str) -> Result<()> {
    let probes = create_example_probes();
    let json = serde_json::to_string_pretty(&probes)?;
    std::fs::write(output, json)?;
    
    println!("📋 Generated example probes configuration: {}", output);
    println!("🔧 Edit this file to customize your AST probes");
    
    // Print probe descriptions
    println!("\n📖 Generated Probes:");
    for probe in &probes {
        println!("  🔍 {}: {:?} -> {:?}", probe.name, probe.node_type, probe.action);
    }
    
    Ok(())
}

fn list_probe_types() {
    println!("🔍 Available AST Node Types:");
    println!("  • Function    - Function definitions");
    println!("  • Struct      - Struct definitions");
    println!("  • Enum        - Enum definitions");
    println!("  • Impl        - Implementation blocks");
    println!("  • Trait       - Trait definitions");
    println!("  • Module      - Module declarations");
    println!("  • Use         - Use statements");
    println!("  • Const       - Constant definitions");
    println!("  • Static      - Static definitions");
    println!("  • Type        - Type aliases");
    println!("  • Macro       - Macro definitions");
    println!("  • Expr        - Expressions");
    println!("  • Stmt        - Statements");
    println!("  • Pat         - Patterns");
    println!("  • All         - All node types");
    
    println!("\n⚡ Available Probe Actions:");
    println!("  • Log         - Log information about nodes");
    println!("  • AddAttribute - Add attributes to nodes");
    println!("  • WrapFunction - Wrap functions with decorators");
    println!("  • InjectCode  - Inject code at specific positions");
    println!("  • Transform   - Apply macro transformations");
    println!("  • Collect     - Collect data about nodes");
    println!("  • Enhance     - Add enhancement metadata");
    
    println!("\n🎯 Filter Options:");
    println!("  • name_pattern     - Match node names by pattern");
    println!("  • visibility       - Match by visibility (pub, crate, etc.)");
    println!("  • attributes       - Match nodes with specific attributes");
    println!("  • contains_text    - Match nodes containing text");
    println!("  • complexity_threshold - Match nodes above complexity threshold");
    println!("  • layer           - Match by compiler layer (AST, HIR, MIR)");
}

fn analyze_directory(dir: &str, generate: bool) -> Result<()> {
    println!("📊 Analyzing AST patterns in: {}", dir);
    
    let mut function_count = 0;
    let mut struct_count = 0;
    let mut enum_count = 0;
    let mut impl_count = 0;
    let mut complex_functions = Vec::new();
    
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() && 
           entry.path().extension().map_or(false, |ext| ext == "rs") {
            
            let content = std::fs::read_to_string(entry.path())?;
            if let Ok(syntax_tree) = syn::parse_str::<syn::File>(&content) {
                for item in &syntax_tree.items {
                    match item {
                        syn::Item::Fn(func) => {
                            function_count += 1;
                            if func.sig.ident.to_string().len() > 20 {
                                complex_functions.push(func.sig.ident.to_string());
                            }
                        }
                        syn::Item::Struct(_) => struct_count += 1,
                        syn::Item::Enum(_) => enum_count += 1,
                        syn::Item::Impl(_) => impl_count += 1,
                        _ => {}
                    }
                }
            }
        }
    }
    
    println!("📈 Analysis Results:");
    println!("  Functions: {}", function_count);
    println!("  Structs: {}", struct_count);
    println!("  Enums: {}", enum_count);
    println!("  Impls: {}", impl_count);
    println!("  Complex functions: {}", complex_functions.len());
    
    if generate {
        println!("\n🔧 Generating targeted probes based on analysis...");
        
        let mut probes = Vec::new();
        
        // Generate probe for complex functions
        if !complex_functions.is_empty() {
            probes.push(AstProbe {
                name: "monitor_complex_functions".to_string(),
                node_type: split_decls_rs::ast_reflector::AstNodeType::Function,
                filter: split_decls_rs::ast_reflector::ProbeFilter {
                    name_pattern: None,
                    visibility: None,
                    attributes: vec![],
                    contains_text: None,
                    complexity_threshold: Some(8.0),
                    layer: None,
                },
                action: split_decls_rs::ast_reflector::ProbeAction::Log {
                    message: "Complex function detected - consider refactoring".to_string(),
                },
                enabled: true,
                priority: 1,
            });
        }
        
        // Generate probe for public structs
        if struct_count > 10 {
            probes.push(AstProbe {
                name: "document_public_structs".to_string(),
                node_type: split_decls_rs::ast_reflector::AstNodeType::Struct,
                filter: split_decls_rs::ast_reflector::ProbeFilter {
                    name_pattern: None,
                    visibility: Some("pub".to_string()),
                    attributes: vec![],
                    contains_text: None,
                    complexity_threshold: None,
                    layer: None,
                },
                action: split_decls_rs::ast_reflector::ProbeAction::AddAttribute {
                    attr: "#[doc = \"Auto-generated documentation\"]".to_string(),
                },
                enabled: true,
                priority: 2,
            });
        }
        
        let json = serde_json::to_string_pretty(&probes)?;
        std::fs::write("generated_probes.json", json)?;
        println!("💾 Generated probes saved to: generated_probes.json");
    }
    
    Ok(())
}
