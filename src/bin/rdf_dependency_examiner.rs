use std::collections::HashMap;
use anyhow::Result;

macro_rules! includedeps {
    () => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/dependency_data.rs"));
    };
}

includedeps!();

#[derive(Debug)]
struct RdfTriple {
    subject: String,
    predicate: String,
    object: String,
}

struct DependencyRdf {
    triples: Vec<RdfTriple>,
    dependencies: HashMap<&'static str, Vec<&'static str>>,
}

impl DependencyRdf {
    fn new() -> Self {
        let dependencies = dependency_data!();
        let mut triples = Vec::new();
        
        // Convert dependency data to RDF triples
        for (&decl, deps) in &dependencies {
            for &dep in deps {
                triples.push(RdfTriple {
                    subject: decl.to_string(),
                    predicate: "depends_on".to_string(),
                    object: dep.to_string(),
                });
            }
        }
        
        Self { triples, dependencies }
    }
    
    fn query_dependencies(&self, decl: &str) -> Vec<&str> {
        self.triples.iter()
            .filter(|t| t.subject == decl && t.predicate == "depends_on")
            .map(|t| t.object.as_str())
            .collect()
    }
    
    fn query_dependents(&self, decl: &str) -> Vec<&str> {
        self.triples.iter()
            .filter(|t| t.object == decl && t.predicate == "depends_on")
            .map(|t| t.subject.as_str())
            .collect()
    }
    
    fn examine(&self, decl: &str) {
        println!("🔍 Examining: {}", decl);
        
        let deps = self.query_dependencies(decl);
        if !deps.is_empty() {
            println!("  📥 Dependencies: {:?}", deps);
        }
        
        let dependents = self.query_dependents(decl);
        if !dependents.is_empty() {
            println!("  📤 Dependents: {:?}", dependents);
        }
        
        if deps.is_empty() && dependents.is_empty() {
            println!("  ⚪ No dependencies found");
        }
    }
}

fn main() -> Result<()> {
    let rdf = DependencyRdf::new();
    
    println!("🧠 RDF Dependency Examiner");
    println!("📊 Loaded {} triples from {} declarations", 
             rdf.triples.len(), rdf.dependencies.len());
    
    // Example examinations
    println!("\n🔍 Sample examinations:");
    rdf.examine("Lookup");
    rdf.examine("impl_7");
    rdf.examine("SmallCStr");
    
    println!("\n💬 Interactive mode - type declaration names:");
    println!("Commands: 'list' (show sample), 'stats', 'quit'");
    
    loop {
        print!("> ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "quit" => break,
            "list" => {
                let sample: Vec<_> = rdf.dependencies.keys().take(10).collect();
                println!("📋 Sample declarations: {:?}", sample);
            }
            "stats" => {
                println!("📊 Stats:");
                println!("  Declarations: {}", rdf.dependencies.len());
                println!("  Triples: {}", rdf.triples.len());
                let avg_deps = rdf.triples.len() as f32 / rdf.dependencies.len() as f32;
                println!("  Avg dependencies per decl: {:.2}", avg_deps);
            }
            name => rdf.examine(name),
        }
    }
    
    Ok(())
}
