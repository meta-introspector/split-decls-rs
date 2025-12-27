use crate::syn_type_discovery::SynTypeDiscovery;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("🎯 SELF-DISCOVERING AST GENERATOR");
    println!("Querying codebase to extract live syn types and generate visitor code\n");

    let mut discovery = SynTypeDiscovery::new();
    
    // Discover types from actual syn source
    println!("🔍 DISCOVERING TYPES FROM SYN SOURCE...");
    discovery.discover_from_syn_source()?;
    
    // Report what we found
    discovery.report_discovery();
    
    // Generate the visitor macro based on discovered types
    println!("\n🔧 GENERATING VISITOR MACRO...");
    let visitor_macro = discovery.generate_visitor_macro();
    fs::write("generated_visitor_macro.rs", &visitor_macro)?;
    println!("  Saved to: generated_visitor_macro.rs");
    
    // Generate QA assertions
    println!("\n✅ GENERATING QA ASSERTIONS...");
    let qa_assertions = discovery.generate_qa_assertions();
    fs::write("generated_qa_assertions.rs", &qa_assertions)?;
    println!("  Saved to: generated_qa_assertions.rs");
    
    // Generate proof report
    println!("\n📊 GENERATING PROOF REPORT...");
    let proof_report = generate_proof_report(&discovery);
    fs::write("discovery_proof_report.md", &proof_report)?;
    println!("  Saved to: discovery_proof_report.md");
    
    println!("\n🎉 SELF-DISCOVERY COMPLETE!");
    println!("Generated visitor code covers {} discovered types", discovery.discovered_types.len());
    
    Ok(())
}

fn generate_proof_report(discovery: &SynTypeDiscovery) -> String {
    let mut report = String::from("# Self-Discovering AST Generator Proof Report\n\n");
    
    report.push_str("## Discovery Summary\n\n");
    report.push_str(&format!("- **Discovered Types**: {}\n", discovery.discovered_types.len()));
    report.push_str(&format!("- **Visit Methods**: {}\n", discovery.visit_methods.len()));
    report.push_str(&format!("- **Enum Variants**: {}\n", discovery.enum_variants.len()));
    
    report.push_str("\n## Discovered Types\n\n");
    let mut types: Vec<_> = discovery.discovered_types.iter().collect();
    types.sort();
    for type_name in types {
        report.push_str(&format!("- `{}`\n", type_name));
    }
    
    report.push_str("\n## Generated Visit Methods\n\n");
    let mut methods: Vec<_> = discovery.visit_methods.iter().collect();
    methods.sort();
    for method in methods {
        report.push_str(&format!("- `{}`\n", method));
    }
    
    report.push_str("\n## QA Assertions\n\n");
    report.push_str("The generated code includes assertions that verify:\n");
    report.push_str("1. All expected core types are discovered\n");
    report.push_str("2. Minimum type coverage is achieved\n");
    report.push_str("3. Generated visitor methods match discovered types\n");
    
    report.push_str("\n## Self-Application Proof\n\n");
    report.push_str("This system demonstrates self-application by:\n");
    report.push_str("1. **Querying its own codebase** to discover live types\n");
    report.push_str("2. **Generating visitor code** based on actual findings\n");
    report.push_str("3. **Creating QA tests** to verify completeness\n");
    report.push_str("4. **Producing this proof report** as evidence\n");
    
    report
}
