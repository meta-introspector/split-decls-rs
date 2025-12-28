use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
    let mut generated_code = String::new();
    
    // Tracing macro that wraps function calls
    generated_code.push_str("#[macro_export]\n");
    generated_code.push_str("macro_rules! trace_call {\n");
    generated_code.push_str("    ($func:ident($($arg:expr),*)) => {\n");
    generated_code.push_str("        {\n");
    generated_code.push_str("            println!(\"🔧 TRACE: Calling {}({})\", stringify!($func), stringify!($($arg),*));\n");
    generated_code.push_str("            let start = std::time::Instant::now();\n");
    generated_code.push_str("            let result = $func($($arg),*);\n");
    generated_code.push_str("            let duration = start.elapsed();\n");
    generated_code.push_str("            match &result {\n");
    generated_code.push_str("                Ok(_) => println!(\"✅ TRACE: {} completed in {:?}\", stringify!($func), duration),\n");
    generated_code.push_str("                Err(e) => println!(\"❌ TRACE: {} failed in {:?}: {}\", stringify!($func), duration, e),\n");
    generated_code.push_str("            }\n");
    generated_code.push_str("            result\n");
    generated_code.push_str("        }\n");
    generated_code.push_str("    };\n");
    generated_code.push_str("}\n\n");
    
    // Bootstrap wrapper macro
    generated_code.push_str("#[macro_export]\n");
    generated_code.push_str("macro_rules! traced_bootstrap {\n");
    generated_code.push_str("    () => {\n");
    generated_code.push_str("        {\n");
    generated_code.push_str("            println!(\"🚀 TRACED BOOTSTRAP EXECUTION START\");\n");
    generated_code.push_str("            let scan_root = std::path::Path::new(\"../../\");\n");
    generated_code.push_str("            \n");
    generated_code.push_str("            // Call the real bootstrap functions with tracing\n");
    generated_code.push_str("            trace_call!(split_decls_rs::run_bootstrap_mode(scan_root, false, false, true))\n");
    generated_code.push_str("        }\n");
    generated_code.push_str("    };\n");
    generated_code.push_str("}\n");
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    f.write_all(generated_code.as_bytes()).unwrap();
}
