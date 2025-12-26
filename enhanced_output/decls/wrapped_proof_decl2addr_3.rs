// Generated from: ./src/bin/proof_decl2addr.rs
// Original file: ./src/bin/proof_decl2addr.rs
// Function: get_binary_address

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

#[decl_split_decls_rs_proof_decl2addr]
# [doc = " Get address from binary symbols"] fn get_binary_address (symbol : & str) -> Result < String > { let output = Command :: new ("readelf") . arg ("-s") . arg ("/proc/self/exe") . output () ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; for line in stdout . lines () { if line . contains (symbol) { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () >= 2 { let addr = parts [1] ; if addr != "0" && addr . len () >= 8 { return Ok (format ! ("0x{}" , addr)) ; } } } } Err (anyhow :: anyhow ! ("Binary symbol not found")) }