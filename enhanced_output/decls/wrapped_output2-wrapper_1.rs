// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: precompile_bootstrap

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

#[decl_split_decls_rs_output2-wrapper]
fn precompile_bootstrap (rdf_state : & mut RdfStateMachine , output : & PathBuf , verbose : bool) -> anyhow :: Result < () > { rdf_state . enter_function ("precompile_bootstrap") ; println ! ("🎯 BOOTSTRAP PRECOMPILE STEPS:") ; println ! ("  1. Load split-decls-rs source") ; println ! ("  2. Apply wrapped syn analysis") ; println ! ("  3. Generate compressed AST") ; println ! ("  4. Output to {}" , output . display ()) ; let bootstrap_files = ["src/main.rs" , "src/lib.rs" , "src/macro_interpreter.rs" , "src/ast_statistics.rs" ,] ; for file_path in & bootstrap_files { if let Ok (code) = std :: fs :: read_to_string (file_path) { let result = interpret_syn_function ! (rdf_state , "syn::parse_file" , "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse_file.rs") ; rdf_state . capture_data ("analyzed_file" , file_path) ; rdf_state . capture_data ("file_size" , & code . len () . to_string ()) ; if verbose { println ! ("    Analyzed: {} ({} bytes)" , file_path , code . len ()) ; } } } rdf_state . exit_function ("precompile_bootstrap") ; println ! ("✅ Bootstrap precompile complete") ; Ok (()) }