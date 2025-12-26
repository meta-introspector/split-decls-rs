// Generated from: ./src/bin/proof_decl2addr.rs
// Original file: ./src/bin/proof_decl2addr.rs
// Function: hash_to_addr

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
# [doc = " Hash to address (deterministic fallback)"] fn hash_to_addr (name : & str) -> u64 { let mut hasher = std :: collections :: hash_map :: DefaultHasher :: new () ; std :: hash :: Hasher :: write (& mut hasher , name . as_bytes ()) ; std :: hash :: Hasher :: finish (& hasher) }