// Generated from: ./src/bin/analyze_common_terms.rs
// Original file: ./src/bin/analyze_common_terms.rs
// Function: get_rust_symbol_address

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

#[decl_split_decls_rs_analyze_common_terms]
fn get_rust_symbol_address (symbol : & str) -> Result < String > { let mangled_patterns = vec ! [format ! ("_ZN*{}*" , symbol) , format ! ("{}*" , symbol) , format ! ("*{}*" , symbol) ,] ; let output = Command :: new ("nm") . arg ("/proc/self/exe") . output () ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; for line in stdout . lines () { for pattern in & mangled_patterns { if line . contains (symbol) && line . contains ("T ") { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () >= 1 && parts [0] != "0000000000000000" { return Ok (format ! ("0x{}" , parts [0])) ; } } } } Err (anyhow :: anyhow ! ("Not found")) }