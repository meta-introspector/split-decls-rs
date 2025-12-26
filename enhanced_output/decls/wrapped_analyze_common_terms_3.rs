// Generated from: ./src/bin/analyze_common_terms.rs
// Original file: ./src/bin/analyze_common_terms.rs
// Function: get_real_address

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
# [doc = " Get real memory address using multiple methods"] fn get_real_address (symbol : & str) -> String { if let Ok (addr) = get_nm_address (symbol) { return addr ; } if let Ok (addr) = get_objdump_address (symbol) { return addr ; } if let Ok (addr) = get_rust_symbol_address (symbol) { return addr ; } format ! ("0x{:x}(hash)" , hash_symbol (symbol)) }