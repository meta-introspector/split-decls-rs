use anyhow::{Context, Result};
use crate::CratePaths;
use crate::SplitDeclsConfig;
use proc_macro2::{Ident, TokenStream, Span};
use crate::eager_splitter::ModuleNotFoundReport;
use syn::LitStr;
use std::fs;
use quote::quote;
use crate::rustfmt_utils::format_rust_file;
use crate::add_generated_rust_header;
use crate::eager_splitter::declaration_writer;
use crate::eager_splitter::declaration_extractor;

     //process_module_recursively
pub fn process_module_recursively(
    paths: &CratePaths,
    config: &SplitDeclsConfig,
    mod_name: &str,
    parent_path: &str,
    collected_module_names: &mut Vec<Ident>,
    item_count: &mut usize,
    common_uses: &TokenStream,
    dry_run: bool,
    module_not_found_errors: &mut Vec<ModuleNotFoundReport>, // New parameter
) -> Result<()> {
    let src_dir = paths.crate_path.join("src");
    let mod_file1 = src_dir.join(format!("{}.rs", mod_name));
    let mod_file2 = src_dir.join(mod_name).join("mod.rs");
    
    let mod_file = if mod_file1.exists() {
        mod_file1
    } else if mod_file2.exists() {
        mod_file2
    } else {
        // Collect the error instead of printing
        let crate_name_sanitized = paths.crate_name.replace("-", "_").replace(".", "_");
        let error_message = format!("Module file not found for: {}", mod_name);
        let error_module_name_str = format!("{}_decls_module_not_found_{}", crate_name_sanitized, mod_name);
        let decl_file_path = paths.decls_output_dir.join(format!("{}.rs", error_module_name_str));

        module_not_found_errors.push(ModuleNotFoundReport {
            crate_name: crate_name_sanitized.clone(),
            module_name: mod_name.to_string(),
            error_message: error_message.clone(),
            generated_file_path: decl_file_path.clone(),
        });
        
        let error_module_name_ident = Ident::new(&error_module_name_str, Span::call_site());
        collected_module_names.push(error_module_name_ident.clone());

        let error_message_lit = LitStr::new(&error_message, Span::call_site());
        let crate_name_sanitized_lit = LitStr::new(&crate_name_sanitized, Span::call_site());
        let mod_name_lit = LitStr::new(mod_name, Span::call_site());

        let error_output_tokens = quote! {
            #[llm_error_message(
                message = #error_message_lit
            )]
            #[llm_context(
                crate_name = #crate_name_sanitized_lit,
                module_name = #mod_name_lit
            )]
            pub struct #error_module_name_ident;
        };

        if !dry_run {
            add_generated_rust_header!(
                &decl_file_path,
                error_output_tokens.to_string().as_str(),
                file!(),
                line!()
            )
            .context(format!("Failed to write module not found error to {}", decl_file_path.display()))?;
            format_rust_file(&decl_file_path)?;
        } else {
            // No need to print in dry-run, as it will be in the summary
        }

        *item_count += 1;
        return Ok(());
    };
    
    let mod_content = fs::read_to_string(&mod_file)
        .context(format!("Failed to read module file: {}", mod_file.display()))?;

    let crate_name_sanitized = paths.crate_name.replace("-", "_").replace(".", "_");

    let mod_ast = match syn::parse_str::<syn::File>(&mod_content) {
        Ok(ast) => ast,
        Err(e) => {
            let mod_file_display = mod_file.display();
            eprintln!("Error parsing module file {}: {}", mod_file_display, e);

            let file_stem = mod_file.file_stem().unwrap().to_string_lossy();
            let error_module_name_str = format!("{}_decls_error_{}", crate_name_sanitized, file_stem.replace("-", "_").replace(".", "_"));
            let error_module_name_ident = Ident::new(&error_module_name_str, Span::call_site());
            collected_module_names.push(error_module_name_ident.clone());

            let error_message = e.to_string();
            let error_line = e.span().start().line;
            let error_column = e.span().start().column;
            
            let file_path_str_lit = LitStr::new(&mod_file_display.to_string(), Span::call_site());
            let error_message_lit = LitStr::new(&error_message, Span::call_site());
            let file_content_lit = LitStr::new(&mod_content, Span::call_site());
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
                pub struct #error_module_name_ident;
            };

            let decl_file_path = paths.decls_output_dir.join(format!("{}.rs", error_module_name_str));
            if !dry_run {
                add_generated_rust_header!(
                    &decl_file_path,
                    error_output_tokens.to_string().as_str(),
                    file!(),
                    line!()
                )
                .context(format!("Failed to write error declaration to {}", decl_file_path.display()))?;
                format_rust_file(&decl_file_path)?;
            } else {
                println!("Dry-run: Would write error declaration to {}", decl_file_path.display());
            }
            println!("  (Parsing error captured in {} for later LLM processing)", decl_file_path.display());

            *item_count += 1;
            return Ok(());
        }
    };
    
    for item in &mod_ast.items {
        if let Some(decl) = declaration_extractor::extract_single_declaration(item, *item_count) {
            let full_path = if parent_path.is_empty() {
                mod_name.to_string()
            } else {
                format!("{}_{}", parent_path, mod_name)
            };
            
            let module_name_str = format!("{}_decls_{}_{}", 
                crate_name_sanitized, 
                full_path.replace("-", "_").replace(".", "_"),
                decl.name
            );
            let module_name_ident = Ident::new(&module_name_str, Span::call_site());
            collected_module_names.push(module_name_ident.clone());
            
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
        }
        
        // Recursively process nested modules
        if let syn::Item::Mod(item_mod) = item {
            let nested_mod_name = item_mod.ident.to_string();
            let new_parent_path = if parent_path.is_empty() {
                mod_name.to_string()
            } else {
                format!("{}_{}", parent_path, mod_name)
            };
            
            process_module_recursively(
                paths,
                config,
                &nested_mod_name,
                new_parent_path.as_str(), // Changed &new_parent_path to new_parent_path.as_str()
                collected_module_names,
                item_count,
                common_uses,
                dry_run,
                module_not_found_errors, // Pass the new parameter
            )?;
        }
        if let syn::Item::Mod(item_mod) = item {
            for attr in &item_mod.attrs {
                if attr.path().is_ident("path") {
                    if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                        let path_str = lit.value();
                        let source_path = mod_file.parent().unwrap().join(&path_str);
                        let dest_path = paths.decls_output_dir.join(&path_str);
                        if source_path.exists() {
                            if let Some(parent) = dest_path.parent() {
                                fs::create_dir_all(parent)?;
                            }
                            fs::copy(&source_path, &dest_path)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}    
    
