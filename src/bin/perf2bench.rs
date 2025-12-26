use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug)]
struct PerfFunction {
    percentage: f64,
    binary: String,
    function: String,
}

#[derive(Debug)]
struct BenchMacro {
    perf_id: String,
    percentage: f64,
    wrap_path: Option<String>,
}

fn parse_perf_functions(perf_file: &str) -> Result<Vec<PerfFunction>> {
    let content = fs::read_to_string(perf_file)?;
    let mut functions = Vec::new();
    
    for line in content.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 4 && parts[0].ends_with('%') {
            if let Ok(percentage) = parts[0].trim_end_matches('%').parse::<f64>() {
                let binary = parts[1].to_string();
                let function = parts[3..].join(" ");
                functions.push(PerfFunction {
                    percentage,
                    binary,
                    function,
                });
            }
        }
    }
    
    Ok(functions)
}

fn find_matching_decl(function_name: &str, output2_path: &Path) -> Option<String> {
    // Extract crate name from function (e.g., "syn::token::parsing::peek_punct" -> "syn")
    let crate_name = function_name.split("::").next()?;
    let wrapped_dir = output2_path.join(format!("wrapped-{}", crate_name));
    
    if !wrapped_dir.exists() {
        return None;
    }
    
    // Look for matching declaration file
    let decls_dir = wrapped_dir.join("src/decls");
    if let Ok(entries) = fs::read_dir(&decls_dir) {
        for entry in entries.flatten() {
            let file_name_string = entry.file_name().to_string_lossy().to_string();
            if file_name_string.contains(&function_name.replace("::", "_")) {
                return Some(entry.path().to_string_lossy().to_string());
            }
        }
    }
    
    None
}

fn generate_bench_macros(functions: Vec<PerfFunction>, output2_path: &Path) -> Vec<BenchMacro> {
    functions
        .into_iter()
        .map(|func| {
            let wrap_path = find_matching_decl(&func.function, output2_path);
            BenchMacro {
                perf_id: func.function.clone(),
                percentage: func.percentage,
                wrap_path,
            }
        })
        .collect()
}

fn main() -> Result<()> {
    let perf_file = "user_functions.txt";
    let output2_path = Path::new("output2");
    
    let functions = parse_perf_functions(perf_file)?;
    let bench_macros = generate_bench_macros(functions, output2_path);
    
    println!("// Generated perf2bench report");
    println!("// Maps perf hotspots to output2 macro declarations\n");
    
    for macro_call in bench_macros {
        match macro_call.wrap_path {
            Some(path) => {
                println!("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"{}\")",
                    macro_call.perf_id,
                    macro_call.percentage,
                    path
                );
            }
            None => {
                println!("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"NOT_FOUND\")",
                    macro_call.perf_id,
                    macro_call.percentage
                );
            }
        }
    }
    
    Ok(())
}
