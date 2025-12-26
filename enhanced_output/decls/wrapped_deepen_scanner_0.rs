// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
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

#[decl_split_decls_rs_deepen_scanner]
fn main () -> Result < () > { println ! ("🔍 Deepen Scanner - Self-Analysis Mode") ; let mut tape_macros = HashMap :: new () ; let self_code = fs :: read_to_string ("src/bin/deepen_scanner.rs") ? ; tape_macros . insert ("deepen_scanner_self" . to_string () , self_code) ; println ! ("📼 Loaded {} macros from tape (self-code)" , tape_macros . len ()) ; let output2_blocks = scan_output2_blocks () ? ; println ! ("📂 Found {} blocks in output2" , output2_blocks . len ()) ; let similarities = find_similarities (& tape_macros , & output2_blocks) ? ; print_similarity_report (& similarities) ; Ok (()) }