use syn::{parse_file, Item};
use std::fs;
use std::path::Path;

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

fn process_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let ast = parse_file(&content)?;
    
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
    
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Building with macro wrappers...");
    
    // Process a few test files first
    let test_files = [
        "submodules/rust/compiler/rustc/build.rs",
        "submodules/rust/compiler/rustc/src/main.rs",
        "submodules/rust/compiler/rustc_driver_impl/src/lib.rs",
    ];
    
    for file_path in &test_files {
        if Path::new(file_path).exists() {
            match process_file(file_path) {
                Ok(wrapped_content) => {
                    let output_path = format!("processed_{}", file_path.replace("/", "_"));
                    fs::write(&output_path, wrapped_content)?;
                    println!("✅ Processed: {} -> {}", file_path, output_path);
                }
                Err(e) => {
                    println!("❌ Failed to process {}: {}", file_path, e);
                }
            }
        }
    }
    
    Ok(())
}
