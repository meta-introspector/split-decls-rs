// Generated from: ./src/bin/proof_decl2addr.rs
// Original file: ./src/bin/proof_decl2addr.rs
// Function: real_address_lookup

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
# [doc = " Get real memory address of a declaration"] fn real_address_lookup (decl_name : & str) -> String { if let Ok (addr) = get_symbol_address (decl_name) { return addr ; } if let Ok (addr) = get_runtime_address (decl_name) { return addr ; } if let Ok (addr) = get_binary_address (decl_name) { return addr ; } format ! ("0x{:x}" , hash_to_addr (decl_name)) }