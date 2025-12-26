// Generated from: ./src/bin/test_addr2line.rs
// Original file: ./src/bin/test_addr2line.rs
// Function: eval_declaration

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

#[decl_split_decls_rs_test_addr2line]
# [doc = " Evaluate a declaration as a macro call"] fn eval_declaration (expr : & str , decl : & split_decls_rs :: output2_macro_system :: MacroDeclaration) -> Result < String > { if (expr . starts_with ("(call ") || expr . starts_with ("(invoke ")) && expr . ends_with (")") { let start_pos = if expr . starts_with ("(call ") { 6 } else { 8 } ; let inner = & expr [start_pos .. expr . len () - 1] ; let parts : Vec < & str > = inner . split_whitespace () . collect () ; if parts . len () >= 2 { let macro_name = parts [0] ; let arg = parts [1] . trim_matches ('"') ; match decl . declaration_type . as_str () { "function" => Ok (format ! ("Called function {} with {}" , macro_name , arg)) , "struct" => Ok (format ! ("Instantiated struct {} with {}" , macro_name , arg)) , "enum" => Ok (format ! ("Matched enum {} variant {}" , macro_name , arg)) , "type" => Ok (format ! ("Used type alias {} for {}" , macro_name , arg)) , "impl" => Ok (format ! ("Invoked impl {} method with {}" , macro_name , arg)) , _ => Ok (format ! ("Executed {} ({}) with {}" , macro_name , decl . declaration_type , arg)) , } } else { Err (anyhow :: anyhow ! ("Invalid expression format")) } } else { Err (anyhow :: anyhow ! ("Unsupported expression format")) } }