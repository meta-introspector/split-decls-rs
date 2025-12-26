// Generated from: ./src/bin/proof_wrap_bin.rs
// Original file: ./src/bin/proof_wrap_bin.rs
// Function: extract_address

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

#[decl_split_decls_rs_proof_wrap_bin]
# [doc = " Extract address from objdump line"] fn extract_address (line : & str) -> Option < String > { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () > 0 { let addr = parts [0] ; if addr . len () >= 8 && addr . chars () . all (| c | c . is_ascii_hexdigit ()) { return Some (format ! ("0x{}" , addr)) ; } } None }