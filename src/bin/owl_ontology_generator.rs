use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::Result;

#[derive(Debug)]
struct OwlOntology {
    classes: HashSet<String>,
    properties: HashSet<String>,
    individuals: HashSet<String>,
    triples: Vec<(String, String, String)>, // subject, predicate, object
    domains: HashMap<String, String>, // property -> domain class
    ranges: HashMap<String, String>,  // property -> range class
}

fn main() -> Result<()> {
    let lmdfb_data = fs::read_to_string("lmdfb_lattice_mapping.json")?;
    let lmdfb: serde_json::Value = serde_json::from_str(&lmdfb_data)?;
    
    let mut ontology = OwlOntology {
        classes: HashSet::new(),
        properties: HashSet::new(),
        individuals: HashSet::new(),
        triples: Vec::new(),
        domains: HashMap::new(),
        ranges: HashMap::new(),
    };
    
    // Define base classes
    ontology.classes.insert("LatticeNode".to_string());
    ontology.classes.insert("Integer".to_string());
    ontology.classes.insert("Symbol".to_string());
    ontology.classes.insert("Layer".to_string());
    ontology.classes.insert("Crate".to_string());
    ontology.classes.insert("Declaration".to_string());
    
    // Define layer subclasses
    ontology.classes.insert("FoundationLayer".to_string());
    ontology.classes.insert("SystemLayer".to_string());
    ontology.classes.insert("CompilerLayer".to_string());
    
    // Define integer type classes
    ontology.classes.insert("PowerOfTwo".to_string());
    ontology.classes.insert("SmallInteger".to_string());
    ontology.classes.insert("ErrorCode".to_string());
    ontology.classes.insert("CryptoConstant".to_string());
    
    // Define properties
    let properties = [
        ("hasLevel", "LatticeNode", "Integer"),
        ("hasWeight", "LatticeNode", "Integer"), 
        ("hasComplexity", "LatticeNode", "Integer"),
        ("hasSymbol", "LatticeNode", "Symbol"),
        ("belongsToLayer", "LatticeNode", "Layer"),
        ("belongsToCrate", "LatticeNode", "Crate"),
        ("hasEmoji", "LatticeNode", "Symbol"),
        ("dependsOn", "LatticeNode", "LatticeNode"),
        ("usesInteger", "Declaration", "Integer"),
        ("hasValue", "Integer", "Integer"),
        ("isTypeOf", "Integer", "Integer"),
    ];
    
    for (prop, domain, range) in properties {
        ontology.properties.insert(prop.to_string());
        ontology.domains.insert(prop.to_string(), domain.to_string());
        ontology.ranges.insert(prop.to_string(), range.to_string());
    }
    
    // Process nodes to create triples
    if let Some(nodes) = lmdfb["nodes"].as_object() {
        for (symbol, node) in nodes {
            let node_id = format!("node_{}", sanitize_id(symbol));
            ontology.individuals.insert(node_id.clone());
            
            // Node is a LatticeNode
            ontology.triples.push((node_id.clone(), "rdf:type".to_string(), "LatticeNode".to_string()));
            
            // Symbol property
            ontology.triples.push((node_id.clone(), "hasSymbol".to_string(), format!("\"{}\"", symbol)));
            
            // Level property
            if let Some(level) = node.get("level").and_then(|v| v.as_u64()) {
                let level_id = format!("level_{}", level);
                ontology.individuals.insert(level_id.clone());
                ontology.triples.push((level_id.clone(), "rdf:type".to_string(), "Integer".to_string()));
                ontology.triples.push((level_id.clone(), "hasValue".to_string(), level.to_string()));
                ontology.triples.push((node_id.clone(), "hasLevel".to_string(), level_id));
            }
            
            // Weight property
            if let Some(weight) = node.get("weight").and_then(|v| v.as_f64()) {
                let weight_int = (weight * 100.0) as i32;
                let weight_id = format!("weight_{}", weight_int);
                ontology.individuals.insert(weight_id.clone());
                ontology.triples.push((weight_id.clone(), "rdf:type".to_string(), "Integer".to_string()));
                ontology.triples.push((weight_id.clone(), "hasValue".to_string(), weight_int.to_string()));
                ontology.triples.push((node_id.clone(), "hasWeight".to_string(), weight_id));
            }
            
            // Layer property
            if let Some(layer) = node.get("layer").and_then(|v| v.as_str()) {
                let layer_id = format!("layer_{}", layer);
                ontology.individuals.insert(layer_id.clone());
                let layer_class = match layer {
                    "foundation" => "FoundationLayer",
                    "system" => "SystemLayer", 
                    "compiler" => "CompilerLayer",
                    _ => "Layer"
                };
                ontology.triples.push((layer_id.clone(), "rdf:type".to_string(), layer_class.to_string()));
                ontology.triples.push((node_id.clone(), "belongsToLayer".to_string(), layer_id));
            }
            
            // Emoji property
            if let Some(emoji) = node.get("emoji").and_then(|v| v.as_str()) {
                ontology.triples.push((node_id.clone(), "hasEmoji".to_string(), format!("\"{}\"", emoji)));
            }
            
            // Extract crate name
            if let Some(crate_name) = symbol.split("::").next() {
                let crate_id = format!("crate_{}", sanitize_id(crate_name));
                ontology.individuals.insert(crate_id.clone());
                ontology.triples.push((crate_id.clone(), "rdf:type".to_string(), "Crate".to_string()));
                ontology.triples.push((node_id.clone(), "belongsToCrate".to_string(), crate_id));
            }
            
            // Extract integers from symbol and create usage relations
            let integers = extract_integers_from_symbol(symbol);
            for integer in integers {
                let int_id = format!("int_{}", integer);
                ontology.individuals.insert(int_id.clone());
                
                // Classify integer type
                let int_type = classify_integer_type(integer);
                ontology.triples.push((int_id.clone(), "rdf:type".to_string(), int_type));
                ontology.triples.push((int_id.clone(), "hasValue".to_string(), integer.to_string()));
                ontology.triples.push((node_id.clone(), "usesInteger".to_string(), int_id));
            }
        }
    }
    
    // Generate OWL/RDF output
    generate_owl_output(&ontology)?;
    
    println!("🦉 OWL Ontology Generated:");
    println!("   Classes: {}", ontology.classes.len());
    println!("   Properties: {}", ontology.properties.len()); 
    println!("   Individuals: {}", ontology.individuals.len());
    println!("   Triples: {}", ontology.triples.len());
    println!("   Saved to: lmdfb_ontology.owl");
    
    Ok(())
}

fn generate_owl_output(ontology: &OwlOntology) -> Result<()> {
    let mut owl = String::new();
    
    // OWL Header
    owl.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    owl.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    owl.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    owl.push_str("@prefix lmdfb: <http://split-decls-rs.org/lmdfb#> .\n\n");
    
    // Ontology declaration
    owl.push_str("<http://split-decls-rs.org/lmdfb> rdf:type owl:Ontology ;\n");
    owl.push_str("    rdfs:label \"LMDFB Lattice Ontology\" ;\n");
    owl.push_str("    rdfs:comment \"Ontology for Rust ecosystem lattice mapping\" .\n\n");
    
    // Classes
    owl.push_str("# Classes\n");
    for class in &ontology.classes {
        owl.push_str(&format!("lmdfb:{} rdf:type owl:Class .\n", class));
    }
    owl.push_str("\n");
    
    // Properties
    owl.push_str("# Properties\n");
    for property in &ontology.properties {
        owl.push_str(&format!("lmdfb:{} rdf:type owl:ObjectProperty", property));
        if let Some(domain) = ontology.domains.get(property) {
            owl.push_str(&format!(" ;\n    rdfs:domain lmdfb:{}", domain));
        }
        if let Some(range) = ontology.ranges.get(property) {
            owl.push_str(&format!(" ;\n    rdfs:range lmdfb:{}", range));
        }
        owl.push_str(" .\n\n");
    }
    
    // Individuals and triples (sample)
    owl.push_str("# Sample Individuals and Relations\n");
    for (i, (subject, predicate, object)) in ontology.triples.iter().take(100).enumerate() {
        if predicate == "rdf:type" {
            owl.push_str(&format!("lmdfb:{} {} lmdfb:{} .\n", subject, predicate, object));
        } else if object.starts_with('"') {
            owl.push_str(&format!("lmdfb:{} lmdfb:{} {} .\n", subject, predicate, object));
        } else {
            owl.push_str(&format!("lmdfb:{} lmdfb:{} lmdfb:{} .\n", subject, predicate, object));
        }
    }
    
    fs::write("lmdfb_ontology.owl", owl)?;
    Ok(())
}

fn sanitize_id(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

fn extract_integers_from_symbol(symbol: &str) -> Vec<i32> {
    let mut integers = Vec::new();
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
                    integers.push(final_num);
                }
                current_num.clear();
                is_negative = false;
            }
        }
    }
    
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<i32>() {
            let final_num = if is_negative { -num } else { num };
            integers.push(final_num);
        }
    }
    
    integers
}

fn classify_integer_type(value: i32) -> String {
    if value > 0 && (value & (value - 1)) == 0 {
        "PowerOfTwo".to_string()
    } else if value < 0 {
        "ErrorCode".to_string()
    } else if value == 25519 || value == -25519 || value == 256 || value == -256 {
        "CryptoConstant".to_string()
    } else if value >= -10 && value <= 10 {
        "SmallInteger".to_string()
    } else {
        "Integer".to_string()
    }
}
