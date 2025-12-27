use std::time::Instant;
use std::path::Path;
use crate::macro_interpreter::RdfStateMachine;
use crate::{interpret_wrapped_decl, interpret_syn_function};

// Macro to benchmark wrapped syn functions with RDF capture
macro_rules! bench_wrapped_syn_rdf {
    ($rdf_state:expr, $func_name:expr, $wrap_path:expr) => {
        {
            let start = Instant::now();
            let result = interpret_syn_function!($rdf_state, $func_name, $wrap_path);
            let exists = Path::new($wrap_path).exists();
            let status = if exists { "✅" } else { "❌" };
            println!("{} {} -> {}", status, $func_name, result);
            start.elapsed()
        }
    };
}

fn main() {
    println!("🔥 DRIVE-BENCH-PERF-WRAPPED-SYN");
    println!("Benchmarking syn hotspots using macro interpreter with RDF capture\n");

    let mut rdf_state = RdfStateMachine::new();

    // Top syn performance hotspots mapped to actual wrapped declarations
    let syn_hotspots = [
        ("syn::token::parsing::peek_punct", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_token.rs"),
        ("syn::lit::value::parse_lit_str", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_lit.rs"),
        ("syn::buffer::Cursor::group", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_buffer.rs"),
        ("syn::ident::parsing::accept_as_ident", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_ident.rs"),
        ("syn::parse::span_of_unexpected_ignoring_nones", "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse.rs"),
        ("syn::expr::parsing::trailer_helper", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_expr.rs"),
        ("syn::parse::ParseBuffer::step", "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse.rs"),
        ("syn::error::Error::new", "output2/wrapped-syn/src/decls/wrapped_syn_decls_module_not_found_error.rs"),
    ];

    let mut total_time = std::time::Duration::new(0, 0);
    let mut found_count = 0;

    for (func_name, wrap_path) in &syn_hotspots {
        let elapsed = bench_wrapped_syn_rdf!(rdf_state, func_name, wrap_path);
        total_time += elapsed;
        
        if Path::new(wrap_path).exists() {
            found_count += 1;
        }
    }

    println!("\n📊 RESULTS:");
    println!("Found: {}/{} wrapped declarations", found_count, syn_hotspots.len());
    println!("Total benchmark time: {:?}", total_time);
    println!("RDF triples captured: {}", rdf_state.triples.len());
    
    // Output RDF data
    println!("\n🔗 RDF EXECUTION TRACE:");
    for triple in &rdf_state.triples {
        println!("  {} -> {} -> {} ({})", triple.subject, triple.predicate, triple.object, triple.timestamp);
    }
    
    if found_count > 0 {
        println!("\n✅ Macro interpreter ready for full AST execution");
    } else {
        println!("\n❌ Need to locate correct wrapped declarations");
    }
}
