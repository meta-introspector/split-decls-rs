
use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream}; // Added LineColumn
use crate::process_module_recursivly::process_module_recursively;
use quote::ToTokens;
use std::fs;
// use std::io::Write; // commented out because it's no longer used
use std::path::{Path, PathBuf}; // Added Path
use syn::visit::Visit;
use syn::{self, LitStr, Item}; // Added LitStr and Item
use quote::quote; // Added quote
use log::{info, warn, error};
use std::thread;

use crate::paths::CratePaths;
use split_decls_types::SplitDeclsConfig;

// Import new modules
pub mod use_collector;
pub mod declaration_extractor;
pub mod declaration_writer;
pub mod invocation_generator;

/// Recursively finds all .rs files in src directory that cargo would build
fn find_all_rust_files(src_dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut rust_files = Vec::new();
    
    if !src_dir.exists() {
        return Ok(rust_files);
    }
    
    for entry in 
    std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            rust_files.push(path);
        } else if path.is_dir() {
            // Recursively search subdirectories
            rust_files.extend(find_all_rust_files(&path)?);
        }
    }
    
    Ok(rust_files)
}

use std::collections::HashMap; // Added for HashMap
use crate::rustfmt_utils::format_rust_file;
use crate::add_generated_rust_header;

// Struct to hold module not found errors for later reporting
#[derive(Debug, Clone)]
pub struct ModuleNotFoundReport {
    pub crate_name: String,
    pub module_name: String,
    pub error_message: String,
    pub generated_file_path: PathBuf,
}


/// Processes all .rs files found in src directory
fn process_all_rust_files(
    paths: &CratePaths,
    config: &SplitDeclsConfig,
    collected_module_names: &mut Vec<Ident>,
    item_count: &mut usize,
    common_uses: &TokenStream,
    dry_run: bool,
    module_not_found_errors: &mut Vec<ModuleNotFoundReport>, // New parameter
) -> Result<()> {
    thread_local! {
        static RECURSION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
    }
    
    // Check recursion depth
    let current_depth = RECURSION_DEPTH.with(|d| {
        let depth = d.get();
        d.set(depth + 1);
        depth + 1
    });
    
    if current_depth > 1000 {
        eprintln!("🚨 RECURSION DEPTH EXCEEDED: {} levels in crate: {}", current_depth, paths.crate_name);
        return Err(anyhow::anyhow!("Stack overflow prevention: recursion depth {} exceeded", current_depth));
    }
    
    println!("🔄 PROCESSING CRATE: {} (recursion depth: {})", paths.crate_name, current_depth);
    let src_dir = paths.crate_path.join("src");
    let rust_files = find_all_rust_files(&src_dir)?;
    
    println!("📁 FOUND {} RUST FILES TO PROCESS:", rust_files.len());
    for (i, file) in rust_files.iter().enumerate() {
        println!("   {}: {}", i + 1, file.display());
    }
    
    for rust_file in rust_files {
        // Process ALL .rs files including main.rs - EMIT ALL CODE
        if let Some(file_name) = rust_file.file_name() {
            println!("🔍 EXAMINING FILE: {}", rust_file.display());
            // Only skip if it's a generated file, not main.rs
            if file_name.to_string_lossy().starts_with("wrapped_") {
                continue;
            }
        }
        
        // Skip files in decls directories (generated output)
        if rust_file.to_string_lossy().contains("/decls/") {
            continue;
        }
        
        println!("📖 READING FILE: {}", rust_file.display());
        
        // Add detailed file processing debug
        let file_content = match std::fs::read_to_string(&rust_file) {
            Ok(content) => {
                println!("   ✅ READ SUCCESS: {} bytes", content.len());
                content
            },
            Err(e) => {
                println!("   ❌ READ FAILED: {}", e);
                continue;
            }
        };
    
    // Add thread-local tracking for stack overflow debugging
    thread_local! {
        static CURRENT_FILE: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    }
    
    CURRENT_FILE.with(|f| {
        *f.borrow_mut() = format!("{}", rust_file.display());
    });
    
    // Set up a panic hook for this thread to report the current file
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        CURRENT_FILE.with(|f| {
            eprintln!("🚨 STACK OVERFLOW IN FILE: {}", f.borrow());
        });
        eprintln!("💥 Thread panic: {}", panic_info);
        original_hook(panic_info);
    }));
        
        let file_content = fs::read_to_string(&rust_file)
            .context(format!("Failed to read file: {}", rust_file.display()))?;
            
        println!("   📏 File size: {} bytes", file_content.len());
        
        // Sanitize crate name once
        let crate_name_sanitized = paths.crate_name.replace("-", "_").replace(".", "_");
        
        let file_ast = match syn::parse_str::<syn::File>(&file_content) {
            Ok(ast) => {
                println!("   ✅ PARSED SUCCESSFULLY: {} items found", ast.items.len());
                ast
            },
            Err(e) => {
                let file_path_display = rust_file.display();
                eprintln!("  ❌ Error parsing file {}: {}", file_path_display, e);

                let file_stem = rust_file.file_stem().unwrap().to_string_lossy();
                let error_module_name_str = format!("{}_decls_error_{}", crate_name_sanitized, file_stem.replace("-", "_").replace(".", "_"));
                let error_module_name_ident = Ident::new(&error_module_name_str, Span::call_site());
                collected_module_names.push(error_module_name_ident.clone());

                let error_message = e.to_string();
                let error_line = e.span().start().line;
                let error_column = e.span().start().column;
                
                let file_path_str_lit = LitStr::new(&file_path_display.to_string(), Span::call_site());
                let error_message_lit = LitStr::new(&error_message, Span::call_site());
                let file_content_lit = LitStr::new(&file_content, Span::call_site());
                let crate_name_sanitized_lit = LitStr::new(&crate_name_sanitized, Span::call_site());

                let error_output_tokens = quote! {
                    #[llm_error_message(
                        file = #file_path_str_lit,
                        line = #error_line,
                        column = #error_column,
                        message = #error_message_lit
                    )]
                    #[llm_source_code(
                        #file_content_lit
                    )]
                    #[llm_context(
                        crate_name = #crate_name_sanitized_lit,
                        original_file = #file_path_str_lit
                    )]
                    pub struct #error_module_name_ident; // Dummy item to make it a valid Rust module
                };

                let decl_file_path = paths.decls_output_dir.join(format!("{}.rs", error_module_name_str));
                if !dry_run {
                    let initial_content = error_output_tokens.to_string();
                    match format_rust_file(&initial_content, &decl_file_path) {
                        Ok(formatted_content) => {
                            fs::write(&decl_file_path, formatted_content)
                                .context(format!("Failed to write formatted error declaration to {}", decl_file_path.display()))?;
                        },
                        Err(e) => {
                            let error_comment = format!(
                                "// !!! Formatting failed for this error module. The following code is unformatted. !!!\n\
                                // !!! Error: {} !!!\n\n",
                                e
                            );
                            let content_with_error_comment = error_comment + &initial_content;
                            fs::write(&decl_file_path, content_with_error_comment)
                                .context(format!("Failed to write unformatted error declaration with comment to {}", decl_file_path.display()))?;
                            error!("\n<blip style='color:red'>Formatting error for parsing error in '{}' (written to {})</blip>", rust_file.display(), decl_file_path.display());
                        }
                    }
                } else {
                    println!("Dry-run: Would write error declaration to {}", decl_file_path.display());
                }
                println!("  (Parsing error captured in {} for later LLM processing)", decl_file_path.display());

                *item_count += 1; // Increment item_count for the error declaration
                return Ok(()); // Continue processing other files
            }
        };
        
        // Get relative path for naming
        let rel_path = rust_file.strip_prefix(&src_dir).unwrap_or(&rust_file);
        let path_str = rel_path.to_string_lossy().replace("/", "_").replace("\\", "_").replace(".rs", "");
        
        for (item_index, item) in file_ast.items.iter().enumerate() {
            let item_name = match item {
                Item::Fn(item_fn) => format!("fn {}", item_fn.sig.ident),
                Item::Struct(item_struct) => format!("struct {}", item_struct.ident),
                Item::Enum(item_enum) => format!("enum {}", item_enum.ident),
                Item::Trait(item_trait) => format!("trait {}", item_trait.ident),
                Item::Impl(item_impl) => {
                    if let Some((_, path, _)) = &item_impl.trait_ {
                        format!("impl {} for {:?}", quote!(#path), quote!(#item_impl.self_ty))
                    } else {
                        format!("impl {:?}", quote!(#item_impl.self_ty))
                    }
                },
                Item::Mod(item_mod) => format!("mod {}", item_mod.ident),
                Item::Use(_) => "use statement".to_string(),
                Item::Const(item_const) => format!("const {}", item_const.ident),
                Item::Static(item_static) => format!("static {}", item_static.ident),
                Item::Type(item_type) => format!("type {}", item_type.ident),
                _ => "other item".to_string(),
            };
            
            let item_type = match &item {
                syn::Item::Fn(_) => "fn",
                syn::Item::Struct(_) => "struct", 
                syn::Item::Enum(_) => "enum",
                syn::Item::Impl(_) => "impl",
                syn::Item::Trait(_) => "trait",
                syn::Item::Const(_) => "const",
                syn::Item::Static(_) => "static",
                syn::Item::Type(_) => "type",
                syn::Item::Mod(_) => "mod",
                syn::Item::Use(_) => "use",
                syn::Item::Macro(_) => "macro",
                _ => "other",
            };
            
            // println!("      📋 Item {}: {} ({})", item_index + 1, item_name, item_type);
            
            if let Some(decl) = declaration_extractor::extract_single_declaration(item, *item_count) {
                let module_name_str = format!("{}_decls_{}_{}", 
                    crate_name_sanitized, 
                    path_str.replace("-", "_").replace(".", "_"),
                    decl.name.replace("#", "hash").replace("[", "bracket").replace("]", "bracket").replace("(", "paren").replace(")", "paren").replace(" ", "_").replace("-", "_")
                );
                let module_name_ident = Ident::new(&module_name_str, Span::call_site());
                collected_module_names.push(module_name_ident.clone());
                
                println!("         🔄 PROCESSING: {} -> {}.rs", decl.name, module_name_str);
                
                declaration_writer::write_declaration_file(
                    decl.clone(),
                    paths,
                    config,
                    dry_run,
                    common_uses,
                    module_name_ident,
                )?;
                print!("{}, ", decl.name);
                *item_count += 1;
            } else {
                println!("         ⏭️  SKIPPED: {} (unsupported item type)", item_name);
            }
        }
    }
    
    Ok(())
}
   
/// Extracts declarations from a crate's source files and returns them as a map.
pub fn extract_declarations_to_map(paths: &CratePaths) -> Result<HashMap<String, TokenStream>> {
    let mut all_extracted_decls: HashMap<String, TokenStream> = HashMap::new();
    
    // Process each source file found in the crate
    for source_file in &paths.source_files {
        println!("🔍 PROCESSING SOURCE FILE: {}", source_file.display());
        
        let file_content = fs::read_to_string(source_file)
            .context(format!("Failed to read {}", source_file.display()))?;
    
        let syntax_tree: syn::File = syn::parse_file(&file_content)
            .context(format!("Failed to parse {} as Rust code", source_file.display()))?;

        let mut item_count = 0; // for unique names if needed

        for item in &syntax_tree.items {
            if let Some(decl) = declaration_extractor::extract_single_declaration(item, item_count) {
                let unique_key = format!("{}_{}", source_file.file_stem().unwrap().to_string_lossy(), decl.name);
                all_extracted_decls.insert(unique_key, decl.content);
                item_count += 1;
            }
        }
    }
    
    Ok(all_extracted_decls)
}

/// Copies extracted declarations to the specified output directory.
pub fn copy_declarations_to_output(
    crate_name: &str,
    declarations: &HashMap<String, String>, // Declarations as name -> TokenStream string
    output_base_path: &Path,
) -> Result<()> {
    let crate_output_dir = output_base_path.join(crate_name).join("src").join("decls");
    std::fs::create_dir_all(&crate_output_dir)
        .context(format!("Failed to create output directory for declarations: {}", crate_output_dir.display()))?;

    for (decl_name, decl_tokens_str) in declarations {
        let file_path = crate_output_dir.join(format!("{}.rs", decl_name));
        let initial_content = add_generated_rust_header!(
            decl_tokens_str.as_str(),
            file!(),
            line!()
        );

        match format_rust_file(&initial_content, &file_path) {
            Ok(formatted_content) => {
                
    std::fs::write(&file_path, formatted_content)
                    .context(format!("Failed to write formatted declaration to {}", file_path.display()))?;
            },
            Err(e) => {
                let error_comment = format!(
                    "// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n",
                    e
                );
                let content_with_error_comment = error_comment + &initial_content;
                
    std::fs::write(&file_path, content_with_error_comment)
                    .context(format!("Failed to write unformatted declaration with error comment to {}", file_path.display()))?;
                error!("\n<blip style='color:red'>Formatting error for '{} {}' (written to {})</blip>", "declaration", decl_name, file_path.display());
            }
        }
    }
    Ok(())
}


/// Main entry point for eager splitting of a crate
pub fn eager_split_crate(paths: &CratePaths, config: &SplitDeclsConfig) -> Result<Vec<ModuleNotFoundReport>> {
    println!("🚀 STARTING CRATE PROCESSING: {}", paths.crate_name);
    println!("   📂 Crate path: {}", paths.crate_path.display());
    println!("   📄 Source files: {:?}", paths.source_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>());
    println!("   📁 Output directory: {}", paths.decls_output_dir.display());
    
    let mut module_not_found_errors: Vec<ModuleNotFoundReport> = Vec::new();
    let mut collected_module_names: Vec<Ident> = Vec::new();
    let mut item_count: usize = 0;
    let common_uses = quote! {}; // Empty for now, could be populated from config
    
    // Process ALL .rs files in the crate (including lib.rs and individual modules)
    info!("📖 Processing all .rs files in crate...");
    println!("🔍 SCANNING FOR RUST FILES...");
    process_all_rust_files(
        paths, 
        config, 
        &mut collected_module_names, 
        &mut item_count, 
        &common_uses, 
        false, // dry_run
        &mut module_not_found_errors
    )?;
    
    println!("📊 PROCESSING SUMMARY:");
    println!("   📋 Total items processed: {}", item_count);
    println!("   📦 Total modules generated: {}", collected_module_names.len());
    
    // 4. Generate new lib.rs in output directory
    println!("DEBUG: Before generate_output_lib_rs call.");
    generate_output_lib_rs(paths)?;
    println!("DEBUG: After generate_output_lib_rs call.");
    
    // 5. Generate new lib.rs in original location (for compatibility)
    println!("DEBUG: Before generate_new_lib_rs call.");
    generate_new_lib_rs(paths)?;
    println!("DEBUG: After generate_new_lib_rs call.");
    
    // 6. Generate new build.rs
    println!("DEBUG: Before generate_new_build_rs call.");
    generate_new_build_rs(paths)?;
    println!("DEBUG: After generate_new_build_rs call.");
    
    info!("Eager splitting completed for crate: {}", paths.crate_name);
    info!("Output generated in: {}", paths.decls_output_dir.parent().unwrap().display());
    println!("DEBUG: Exiting eager_split_crate for crate: {}", paths.crate_name);

    Ok(module_not_found_errors)
}

// ...




/// Generate new lib.rs that re-exports the split declarations in the output directory
fn generate_output_lib_rs(paths: &CratePaths) -> Result<()> {
    let output_lib_path = paths.decls_output_dir.parent().unwrap().join("lib.rs");
    let new_lib_content = format!(r#"// Generated by split-decls-rs
// Re-exports all split declarations

pub mod decls {{
    include!("decls/_decl_module_invocation.rs");
}}
pub use decls::*;

// Re-export prelude macros if available

pub use introspector_decl2_macros::*;
"#);
    
    // Ensure the output src directory exists
    fs::create_dir_all(output_lib_path.parent().unwrap())
        .context("Failed to create output src directory")?;
    
    let initial_content = add_generated_rust_header!(
        new_lib_content.as_str(),
        file!(),
        line!()
    );

    match format_rust_file(&initial_content, &output_lib_path) {
        Ok(formatted_content) => {
            fs::write(&output_lib_path, formatted_content)
                .context(format!("Failed to write formatted output lib.rs at {}", output_lib_path.display()))?;
        },
        Err(e) => {
            let error_comment = format!(
                "// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                // !!! Error: {} !!!\n\n",
                e
            );
            let content_with_error_comment = error_comment + &initial_content;
            fs::write(&output_lib_path, content_with_error_comment)
                .context(format!("Failed to write unformatted output lib.rs with error comment at {}", output_lib_path.display()))?;
            error!("\n<blip style='color:red'>Formatting error for output lib.rs (written to {})</blip>", output_lib_path.display());
        }
    }
    
    println!("Generated output lib.rs at {}", output_lib_path.display());
    Ok(())
}

/// Generate new lib.rs that re-exports the split declarations
fn generate_new_lib_rs(paths: &CratePaths) -> Result<()> {
    let output_lib_path = paths.output_crate_path.join("src").join("lib.rs");
    fs::create_dir_all(output_lib_path.parent().unwrap())
        .context(format!("Failed to create output src directory for lib.rs at {}", output_lib_path.display()))?;

    let new_lib_content = format!(r#"// Generated by split-decls-rs
// Re-exports all split declarations

pub mod decls {{
    include!("decls/_decl_module_invocation.rs");
}}
pub use decls::*;

// Re-export prelude macros if available

pub use introspector_decl2_macros::*;
"#);
    
    let initial_content = add_generated_rust_header!(
        new_lib_content.as_str(),
        file!(),
        line!()
    );

    match format_rust_file(&initial_content, &output_lib_path) {
        Ok(formatted_content) => {
            fs::write(&output_lib_path, formatted_content)
                .context(format!("Failed to write formatted new lib.rs at {}", output_lib_path.display()))?;
        },
        Err(e) => {
            let error_comment = format!(
                "// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                // !!! Error: {} !!!\n\n",
                e
            );
            let content_with_error_comment = error_comment + &initial_content;
            fs::write(&output_lib_path, content_with_error_comment)
                .context(format!("Failed to write unformatted new lib.rs with error comment at {}", output_lib_path.display()))?;
            error!("\n<blip style='color:red'>Formatting error for new lib.rs (written to {})</blip>", output_lib_path.display());
        }
    }
    
    println!("Generated new lib.rs at {}", output_lib_path.display());
    Ok(())
}

/// Generate new build.rs for monitoring changes
fn generate_new_build_rs(paths: &CratePaths) -> Result<()> {
    let output_build_path = paths.output_crate_path.join("build.rs");
    fs::create_dir_all(output_build_path.parent().unwrap())
        .context(format!("Failed to create output directory for build.rs at {}", output_build_path.display()))?;

    let build_content = format!(r#"// Generated by split-decls-rs
use anyhow::Result;

fn main() -> Result<()> {{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.split-decls-config.toml");
    
    // This build.rs monitors for changes that would require re-running split-decls-rs
    println!("cargo:note=build.rs finished. If split-decls-rs needs to be re-run, changes will be detected.");
    
    Ok(())
}}
"#);
    
    let initial_content = add_generated_rust_header!(
        build_content.as_str(),
        file!(),
        line!()
    );

    match format_rust_file(&initial_content, &output_build_path) {
        Ok(formatted_content) => {
            fs::write(&output_build_path, formatted_content)
                .context(format!("Failed to write formatted new build.rs at {}", output_build_path.display()))?;
        },
        Err(e) => {
            let error_comment = format!(
                "// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                // !!! Error: {} !!!\n\n",
                e
            );
            let content_with_error_comment = error_comment + &initial_content;
            fs::write(&output_build_path, content_with_error_comment)
                .context(format!("Failed to write unformatted new build.rs with error comment at {}", output_build_path.display()))?;
            error!("\n<blip style='color:red'>Formatting error for new build.rs (written to {})</blip>", output_build_path.display());
        }
    }
    
    println!("Generated new build.rs at {}", output_build_path.display());
    Ok(())
}

/// Performs the eager splitting of the AST into individual declaration files and generates
/// the `_decl_module_invocation.rs` file.
pub fn split_and_generate_decls(
    syntax_tree: &syn::File,
    paths: &CratePaths,
    config: &SplitDeclsConfig,
    dry_run: bool,
    module_not_found_errors: &mut Vec<ModuleNotFoundReport>, // New parameter
) -> Result<()> {
    // Ensure the output directory exists
    if !dry_run {
        fs::create_dir_all(&paths.decls_output_dir)
            .context(format!("Failed to create directory {}", paths.decls_output_dir.display()))?;
        info!("Created directory: {}", paths.decls_output_dir.display()); // Changed println to info
    } else {
        info!(
            "Dry-run: Would create directory: {}",
            paths.decls_output_dir.display()
        ); // Changed println to info
    }

    // Collect use statements
    let mut use_collector_instance = use_collector::UseStatementCollector::default();
    use_collector_instance.visit_file(syntax_tree);
    let common_uses: TokenStream = use_collector_instance.uses.iter().map(|u| u.to_token_stream()).collect();

    let mut collected_module_names: Vec<Ident> = Vec::new();
    let mut item_count = 0; // For generating unique names for impls without explicit paths

    // Extract and split declarations
    info!("🔍 Processing {} items in AST...", syntax_tree.items.len()); // Changed println to info
    for item in &syntax_tree.items {
        if let Some(decl) = declaration_extractor::extract_single_declaration(item, item_count) {
            // Sanitize declaration name to create valid Rust identifier
            let sanitized_name = decl.name.replace("r#", "").replace("#", "_");
            let module_name_str = format!("{}_decls_{}", paths.crate_name.replace("-", "_").replace(".", "_"), sanitized_name);
            let module_name_ident = Ident::new(&module_name_str, Span::call_site());
            collected_module_names.push(module_name_ident.clone());

            declaration_writer::write_declaration_file(
                decl.clone(),
                paths,
                config,
                dry_run,
                &common_uses,
                module_name_ident,
            )?;
            print!("{}, ", decl.name);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
        
        // Process modules recursively
        if let syn::Item::Mod(item_mod) = item {
            let mod_name = item_mod.ident.to_string();
            process_module_recursively(
                paths,
                config,
                &mod_name,
                "",
                &mut collected_module_names,
                &mut item_count,
                &common_uses,
                dry_run,
                module_not_found_errors, // Pass the new parameter
            )?;
        }
        
        item_count += 1;
    }
    
    // Process ALL .rs files in src directory (force include everything cargo would build)
    info!("🔍 Force including all .rs files in src directory..."); // Changed println to info
    process_all_rust_files(
        paths,
        config,
        &mut collected_module_names,
        &mut item_count,
        &common_uses,
        dry_run,
        module_not_found_errors, // Pass the new parameter
    )?;

    // Generate decl_module! invocation
    info!("DEBUG: Collected module names for decl_module!: {:?}", collected_module_names.iter().map(|i| i.to_string()).collect::<Vec<_>>()); // Changed println to info
    invocation_generator::generate_decl_module_invocation(collected_module_names, paths, dry_run)?;

    // Decrement recursion depth on exit
    thread_local! {
        static RECURSION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
    }
    RECURSION_DEPTH.with(|d| {
        let depth = d.get();
        if depth > 0 {
            d.set(depth - 1);
        }
    });

    Ok(())
}
