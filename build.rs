use syn::{parse_file, Item};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};

// Global error type counters for sampling
static ERROR_COUNTERS: LazyLock<Mutex<HashMap<String, usize>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn wrap_item(item: &Item) -> String {
    match item {
        Item::Mod(item_mod) => {
            let mod_name = &item_mod.ident;
            let content = if let Some((_, items)) = &item_mod.content {
                let wrapped_items: Vec<_> = items.iter().map(wrap_item).collect();
                wrapped_items.join("\n")
            } else {
                String::new()
            };
            format!("mkmod!{{{}, {{ 
                getname!({});
                getsrc!({});
                getpath!({});
                get_deps!({});
                get_crates!({});
                mkinclude!({});
                {} 
            }}}}", mod_name, mod_name, mod_name, mod_name, mod_name, mod_name, mod_name, content)
        }
        Item::Use(_) => {
            format!("mkuse!{{{}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Fn(item_fn) => {
            let fn_name = &item_fn.sig.ident;
            format!("
macro_rules! {}_introspect {{
    () => {{
        emit_message!(\"📊 INTROSPECT: Function {} in module {{}}\", module_path!());
    }};
}}

mkfn!{{
    {}_introspect!();
    {}
}}", fn_name, fn_name, fn_name, quote::ToTokens::to_token_stream(item))
        }
        Item::Struct(_) => {
            format!("mkitem!{{mkstruct!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Enum(_) => {
            format!("mkitem!{{mkenum!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Trait(_) => {
            format!("mkitem!{{mktrait!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Impl(_) => {
            format!("mkitem!{{mkimpl!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        _ => format!("mkitem!{{{}}}", quote::ToTokens::to_token_stream(item))
    }
}

fn create_minimal_test_case(file_path: &str, content: &str, error: &dyn std::error::Error) -> Result<(), Box<dyn std::error::Error>> {
    // Extract error type from error message
    let error_msg = error.to_string();
    let error_type = if error_msg.contains("expected square brackets") {
        "expected_square_brackets"
    } else if error_msg.contains("expected identifier") {
        "expected_identifier"  
    } else if error_msg.contains("expected `,`") {
        "expected_comma"
    } else if error_msg.contains("expected an expression") {
        "expected_expression"
    } else if error_msg.contains("unexpected token") {
        "unexpected_token"
    } else {
        "other_parse_error"
    };
    
    // Check if we should create a test case for this error type (max 3 per type)
    let mut counters = ERROR_COUNTERS.lock().unwrap();
    let count = counters.entry(error_type.to_string()).or_insert(0);
    *count += 1;
    
    // Only keep 3 examples of each error type
    if *count > 3 {
        return Ok(()); // Skip this test case
    }
    
    let file_name = file_path.split('/').last().unwrap_or("unknown");
    let test_case_name = format!("test_case_{}_{}.rs", error_type, file_name.replace('.', "_"));
    let test_case_path = format!("test_cases/{}", test_case_name);
    
    // Create test_cases directory if it doesn't exist
    fs::create_dir_all("test_cases")?;
    
    // Try to find the problematic line by parsing line by line
    let lines: Vec<&str> = content.lines().collect();
    let mut minimal_content = String::new();
    let mut error_line = None;
    
    // Try to isolate the error by binary search approach
    for (i, _line) in lines.iter().enumerate() {
        let test_content = lines[0..=i].join("\n");
        if let Err(_) = syn::parse_file(&test_content) {
            error_line = Some(i);
            // Include a few lines around the error for context
            let start = i.saturating_sub(3);
            let end = (i + 4).min(lines.len());
            minimal_content = lines[start..end].join("\n");
            break;
        }
    }
    
    let test_case_content = format!(
        "// MINIMAL TEST CASE for parsing failure in: {}\n\
         // Error: {}\n\
         // Error type: {}\n\
         // Sample #{} of 3\n\
         // Problematic line: {}\n\
         \n\
         {}\n",
        file_path,
        error_msg.lines().next().unwrap_or("unknown error"),
        error_type,
        *count,
        error_line.map_or("unknown".to_string(), |l| format!("line {}", l + 1)),
        minimal_content
    );
    
    fs::write(&test_case_path, test_case_content)?;
    println!("📝 Created test case: {} (sample {}/3)", test_case_name, *count);
    
    Ok(())
}

fn process_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    // Include macro definitions from macro_wrappers.rs
    let macro_defs = fs::read_to_string("src/macro_wrappers.rs")?;
    let content_with_macros = format!("{}\n{}", macro_defs, content);
    
    let ast = parse_file(&content_with_macros)?;
    
    let wrapped_items: Vec<_> = ast.items.iter().map(wrap_item).collect();
    let mut result = wrapped_items.join("\n");
    
    // Fix file paths in the generated content
    result = result.replace("\"../messages.ftl\"", "\"messages.ftl\"");
    
    // Fix environment variable references
    result = result.replace("env ! (\"CFG_RELEASE_CHANNEL\")", "\"dev\"");
    
    // Fix crate-level attributes - remove them completely since they're now in unified_rustc_wrapped.rs
    result = result.replace("# [allow (internal_features)] # [allow (rustc :: untranslatable_diagnostic)] # [doc (html_root_url = \"https://doc.rust-lang.org/nightly/nightly-rustc/\")] # [doc (rust_logo)] # [feature (decl_macro)] # [feature (panic_backtrace_config)] # [feature (panic_update_hook)] # [feature (rustdoc_internals)] # [feature (try_blocks)] ", "");
    
    // Fix inner doc comments - convert //! to //
    result = result.replace("//!", "//");
    
    // Fix attribute spacing - remove spaces in attributes
    result = result.replace("# [", "#[");
    result = result.replace("# !", "#!");
    
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Building with macro wrappers...");
    
    // Load symbol map to find all source files
    if Path::new("symbol_map.json.gz").exists() {
        println!("📊 Loading symbol map...");
        let file = std::fs::File::open("symbol_map.json.gz")?;
        let decoder = flate2::read::GzDecoder::new(file);
        let symbol_map: serde_json::Value = serde_json::from_reader(decoder)?;
        
        if let Some(obj) = symbol_map.as_object() {
            let mut source_files = std::collections::HashSet::new();
            
            // Extract unique source files from symbol map
            for (_, entry) in obj.iter() {
                if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                    if source_file.ends_with(".rs") && !source_file.contains("test") {
                        source_files.insert(source_file.to_string());
                    }
                }
            }
            
            println!("📁 Found {} unique source files", source_files.len());
            
            // Create submodules directory structure
            fs::create_dir_all("submodules")?;
            
            // Process each source file
            for (i, source_file) in source_files.iter().enumerate() {
                let file_path = source_file.replace("../rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/");
                if Path::new(&file_path).exists() {
                    println!("🔄 Processing {}/{}: {}", i+1, source_files.len(), source_file);
                    match process_file(&file_path) {
                        Ok(wrapped_content) => {
                            // Create proper submodules directory structure
                            let output_path = format!("submodules/{}", source_file.replace("../rust/", "rust/"));
                            
                            // Create parent directories
                            if let Some(parent) = Path::new(&output_path).parent() {
                                fs::create_dir_all(parent)?;
                            }
                            
                            fs::write(&output_path, wrapped_content)?;
                            println!("✅ {}/{}: {} -> {}", i+1, source_files.len(), source_file, output_path);
                        }
                        Err(e) => {
                            println!("❌ Failed to process {}: {}", source_file, e);
                            
                            // Create minimal test case for this failure
                            if let Ok(content) = fs::read_to_string(&file_path) {
                                if let Err(test_err) = create_minimal_test_case(&source_file, &content, e.as_ref()) {
                                    println!("⚠️  Failed to create test case: {}", test_err);
                                }
                            }
                        }
                    }
                } else {
                    if i < 10 {
                        println!("⚠️  File not found: {}", file_path);
                    }
                }
            }
        }
    } else {
        println!("❌ symbol_map.json.gz not found. Run: cargo run --bin export_symbol_map");
        return Err("Missing symbol map".into());
    }
    
    Ok(())
}
