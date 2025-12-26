// Generated from: ./src/bin/enhanced_wrapper.rs
// Original file: ./src/bin/enhanced_wrapper.rs
// Function: main

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use split_decls_types::SplitDeclsConfig;
pub use extracted_decl::*;
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;
prelude!{}

#[decl_split_decls_rs_enhanced_wrapper]
fn main () -> Result < () > { let crate_path = Path :: new (".") ; println ! ("🔍 ENHANCED WRAPPING: Scanning ALL Rust files for functions...") ; let all_files = scan_all_rust_files (crate_path) ? ; println ! ("📊 Found {} files with functions:" , all_files . len ()) ; let mut total_functions = 0 ; for (file_path , functions) in & all_files { println ! ("  📁 {}: {} functions" , file_path . display () , functions . len ()) ; total_functions += functions . len () ; } println ! ("🎯 Total functions found: {}" , total_functions) ; println ! ("\n🚀 Generating wrapped declarations for ALL functions...") ; fs :: create_dir_all ("enhanced_output/decls") ? ; let mut generated_count = 0 ; for (file_path , functions) in all_files { for (i , function) in functions . iter () . enumerate () { let wrapped_content = generate_wrapped_decl_with_origin (function , & file_path , "split_decls_rs") ; let file_stem = file_path . file_stem () . and_then (| s | s . to_str ()) . unwrap_or ("unknown") ; let output_path = format ! ("enhanced_output/decls/wrapped_{}_{}.rs" , file_stem , i) ; fs :: write (& output_path , wrapped_content) ? ; generated_count += 1 ; } } println ! ("✅ Generated {} wrapped function declarations" , generated_count) ; println ! ("📁 Output directory: enhanced_output/decls/") ; println ! ("\n🎉 SUCCESS: Complete function wrapping achieved!") ; println ! ("🔥 Every function in the codebase is now addressable and executable!") ; Ok (()) }