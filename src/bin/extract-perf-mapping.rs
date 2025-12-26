use std::fs;
use std::collections::HashMap;

fn main() {
    println!("🔍 EXTRACTING PERF-TO-MODULE MAPPING");
    
    // Read the module invocation file
    let invocation_path = "output2/wrapped-syn/src/decls/_decl_module_invocation.rs";
    let content = fs::read_to_string(invocation_path).expect("Failed to read invocation file");
    
    // Extract module names from decl_module! macro
    let mut modules = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_macro = false;
    
    for line in lines {
        if line.trim().starts_with("decl_module!(") {
            in_macro = true;
            continue;
        }
        if in_macro && line.trim() == ");" {
            break;
        }
        if in_macro {
            let module_name = line.trim().trim_end_matches(',');
            if !module_name.is_empty() {
                modules.push(module_name);
            }
        }
    }
    
    println!("Found {} modules in wrapped-syn", modules.len());
    
    // Map perf functions to modules
    let perf_to_module = [
        ("syn::token::parsing::peek_punct", "wrapped_syn_decls_module_not_found_token"),
        ("syn::lit::value::parse_lit_str", "wrapped_syn_decls_module_not_found_lit"),
        ("syn::buffer::Cursor::group", "wrapped_syn_decls_module_not_found_buffer"),
        ("syn::ident::parsing::accept_as_ident", "wrapped_syn_decls_module_not_found_ident"),
        ("syn::parse::span_of_unexpected_ignoring_nones", "wrapped_syn_decls_parse"),
        ("syn::expr::parsing::trailer_helper", "wrapped_syn_decls_module_not_found_expr"),
        ("syn::parse::ParseBuffer::step", "wrapped_syn_decls_parse"),
        ("syn::error::Error::new", "wrapped_syn_decls_module_not_found_error"),
    ];
    
    println!("\n📊 PERF-TO-MODULE MAPPING:");
    for (perf_func, module_name) in &perf_to_module {
        let file_path = format!("output2/wrapped-syn/src/decls/{}.rs", module_name);
        let exists = std::path::Path::new(&file_path).exists();
        let status = if exists { "✅" } else { "❌" };
        println!("{} {} -> {}", status, perf_func, module_name);
    }
    
    // Generate updated benchmark driver
    println!("\n🔧 GENERATING UPDATED BENCHMARK DRIVER:");
    for (perf_func, module_name) in &perf_to_module {
        let file_path = format!("output2/wrapped-syn/src/decls/{}.rs", module_name);
        println!("(\"{}\", \"{}\"),", perf_func, file_path);
    }
}
