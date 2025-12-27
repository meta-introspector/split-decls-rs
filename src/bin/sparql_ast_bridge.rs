use std::path::Path;
use anyhow::Result;
use clap::{Parser, Subcommand};
use split_decls_rs::sparql_probe_bridge::{SparqlProbeGenerator, create_example_sparql_config};
use split_decls_rs::ast_reflector::AstReflector;

#[derive(Parser)]
#[command(name = "sparql-ast-bridge")]
#[command(about = "SPARQL to AST Probe Bridge - Convert RDF queries to dynamic AST transformations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate probes from SPARQL queries
    Generate {
        /// RDF knowledge base file
        #[arg(short, long, default_value = "rustc_lmdfb_knowledge_base.owl")]
        kb: String,
        
        /// Number of top complex functions to wrap
        #[arg(short, long, default_value = "10")]
        top_n: usize,
        
        /// Output probe file
        #[arg(short, long, default_value = "sparql_generated_probes.json")]
        output: String,
        
        /// Complexity threshold
        #[arg(short, long, default_value = "8.0")]
        complexity: f64,
    },
    
    /// Apply generated probes to directory
    Apply {
        /// Directory to transform
        #[arg(short, long)]
        dir: String,
        
        /// Generated probe file
        #[arg(short, long, default_value = "sparql_generated_probes.json")]
        probes: String,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Generate example SPARQL config
    Config {
        /// Output config file
        #[arg(short, long, default_value = "sparql_config.json")]
        output: String,
    },
    
    /// Interactive mode - query and apply
    Interactive {
        /// RDF knowledge base file
        #[arg(short, long, default_value = "rustc_lmdfb_knowledge_base.owl")]
        kb: String,
        
        /// Target directory
        #[arg(short, long, default_value = "output2")]
        dir: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { kb, top_n, output, complexity } => {
            generate_probes_from_sparql(&kb, top_n, &output, complexity)
        }
        
        Commands::Apply { dir, probes, verbose } => {
            apply_sparql_probes(&dir, &probes, verbose)
        }
        
        Commands::Config { output } => {
            generate_sparql_config(&output)
        }
        
        Commands::Interactive { kb, dir } => {
            run_interactive_mode(&kb, &dir)
        }
    }
}

fn generate_probes_from_sparql(kb_file: &str, top_n: usize, output: &str, complexity_threshold: f64) -> Result<()> {
    println!("🔍 Generating AST probes from SPARQL queries...");
    
    let mut generator = SparqlProbeGenerator::new();
    generator.load_rdf_data(kb_file)?;
    
    // Generate complexity wrapper probes
    let mut all_probes = generator.generate_complexity_wrapper_probes(top_n);
    
    // Add high complexity probes
    let high_complexity_query = split_decls_rs::sparql_probe_bridge::SparqlQuery {
        name: "high_complexity_monitor".to_string(),
        query_type: split_decls_rs::sparql_probe_bridge::QueryType::ComplexityAbove(complexity_threshold),
        threshold: Some(complexity_threshold),
        limit: Some(20),
        target_template: "complexity_wrapper".to_string(),
    };
    
    let template = split_decls_rs::sparql_probe_bridge::ProbeTemplate {
        node_type: split_decls_rs::ast_reflector::AstNodeType::Function,
        action_template: "monitor_complex".to_string(),
        wrapper_function: Some("complexity_ping".to_string()),
        priority: 2,
    };
    
    let high_complexity_probes = generator.generate_probes_from_query(&high_complexity_query, &template);
    all_probes.extend(high_complexity_probes);
    
    // Save probes
    let json = serde_json::to_string_pretty(&all_probes)?;
    
    std::fs::write(output, json)?;
    
    println!("✅ Generated {} probes from SPARQL queries", all_probes.len());
    println!("📊 Probe breakdown:");
    println!("   • Top {} complex functions: {}", top_n, top_n.min(all_probes.len()));
    println!("   • Functions above complexity {}: {}", complexity_threshold, all_probes.len() - top_n.min(all_probes.len()));
    println!("💾 Saved to: {}", output);
    
    // Show sample probes
    println!("\n🔍 Sample generated probes:");
    for (i, probe) in all_probes.iter().take(3).enumerate() {
        println!("   {}. {} -> {:?}", i + 1, probe.name, probe.action);
    }
    
    Ok(())
}

fn apply_sparql_probes(dir: &str, probes_file: &str, verbose: bool) -> Result<()> {
    println!("🚀 Applying SPARQL-generated probes to directory: {}", dir);
    
    let mut reflector = AstReflector::new();
    reflector.load_probes(Path::new(probes_file))?;
    
    if verbose {
        println!("📋 Loaded probes from: {}", probes_file);
    }
    
    reflector.reflect_directory(Path::new(dir))?;
    
    println!("✅ SPARQL probe application completed!");
    Ok(())
}

fn generate_sparql_config(output: &str) -> Result<()> {
    let config = create_example_sparql_config();
    let json = serde_json::to_string_pretty(&config)?;
    
    std::fs::write(output, json)?;
    
    println!("📋 Generated SPARQL configuration: {}", output);
    println!("🔧 Edit this file to customize SPARQL queries and probe templates");
    
    println!("\n📖 Generated queries:");
    for query in &config.queries {
        println!("  🔍 {}: {:?}", query.name, query.query_type);
    }
    
    println!("\n⚡ Generated templates:");
    for (name, template) in &config.probe_templates {
        println!("  📝 {}: {:?} -> {:?}", name, template.node_type, template.wrapper_function);
    }
    
    Ok(())
}

fn run_interactive_mode(kb_file: &str, target_dir: &str) -> Result<()> {
    println!("🎯 Interactive SPARQL-to-AST Mode");
    println!("=================================");
    
    let mut generator = SparqlProbeGenerator::new();
    generator.load_rdf_data(kb_file)?;
    
    loop {
        println!("\n🔍 Available commands:");
        println!("  1. Generate top N complex function probes");
        println!("  2. Generate complexity threshold probes");
        println!("  3. Apply generated probes to {}", target_dir);
        println!("  4. Show current probe statistics");
        println!("  5. Exit");
        
        print!("\nEnter choice (1-5): ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                print!("Enter number of top complex functions (default 10): ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut n_input = String::new();
                std::io::stdin().read_line(&mut n_input)?;
                let n = n_input.trim().parse().unwrap_or(10);
                
                let probes = generator.generate_complexity_wrapper_probes(n);
                let json = serde_json::to_string_pretty(&probes)?;
                
    std::fs::write("interactive_probes.json", json)?;
                
                println!("✅ Generated {} probes for top {} complex functions", probes.len(), n);
            }
            
            "2" => {
                print!("Enter complexity threshold (default 8.0): ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut threshold_input = String::new();
                std::io::stdin().read_line(&mut threshold_input)?;
                let threshold = threshold_input.trim().parse().unwrap_or(8.0);
                
                generate_probes_from_sparql(kb_file, 0, "interactive_probes.json", threshold)?;
            }
            
            "3" => {
                apply_sparql_probes(target_dir, "interactive_probes.json", true)?;
            }
            
            "4" => {
                if let Ok(content) = 
    std::fs::read_to_string("interactive_probes.json") {
                    if let Ok(probes) = serde_json::from_str::<Vec<split_decls_rs::ast_reflector::AstProbe>>(&content) {
                        println!("📊 Current probe statistics:");
                        println!("   Total probes: {}", probes.len());
                        println!("   Enabled probes: {}", probes.iter().filter(|p| p.enabled).count());
                        
                        let mut action_counts = std::collections::HashMap::new();
                        for probe in &probes {
                            let action_type = match &probe.action {
                                split_decls_rs::ast_reflector::ProbeAction::Log { .. } => "Log",
                                split_decls_rs::ast_reflector::ProbeAction::AddAttribute { .. } => "AddAttribute",
                                split_decls_rs::ast_reflector::ProbeAction::WrapFunction { .. } => "WrapFunction",
                                split_decls_rs::ast_reflector::ProbeAction::InjectCode { .. } => "InjectCode",
                                split_decls_rs::ast_reflector::ProbeAction::Transform { .. } => "Transform",
                                split_decls_rs::ast_reflector::ProbeAction::Collect { .. } => "Collect",
                                split_decls_rs::ast_reflector::ProbeAction::Enhance { .. } => "Enhance",
                            };
                            *action_counts.entry(action_type).or_insert(0) += 1;
                        }
                        
                        println!("   Action breakdown:");
                        for (action, count) in action_counts {
                            println!("     • {}: {}", action, count);
                        }
                    }
                } else {
                    println!("❌ No probes generated yet");
                }
            }
            
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            
            _ => {
                println!("❌ Invalid choice, please enter 1-5");
            }
        }
    }
    
    Ok(())
}
