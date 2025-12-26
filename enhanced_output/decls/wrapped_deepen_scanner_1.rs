// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: load_tape_macros

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

#[decl_split_decls_rs_deepen_scanner]
fn load_tape_macros () -> Result < HashMap < String , String > > { let mut macros = HashMap :: new () ; if Path :: new ("repl_state.rdf") . exists () { let rdf_content = fs :: read_to_string ("repl_state.rdf") ? ; for line in rdf_content . lines () { if line . contains ("sys:macro") { if let Some (name) = extract_macro_name (line) { if let Some (content) = extract_macro_content (line) { macros . insert (name , content) ; } } } } } Ok (macros) }