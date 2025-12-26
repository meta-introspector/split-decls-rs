// Generated from: ./src/bin/lmfdb_query.rs
// Original file: ./src/bin/lmfdb_query.rs
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

#[decl_split_decls_rs_lmfdb_query]
fn main () -> Result < () > { let index_content = std :: fs :: read_to_string ("k_theory_index.json") ? ; let index : serde_json :: Value = serde_json :: from_str (& index_content) ? ; if let Some (node) = index ["nodes"] ["k7.1"] . as_object () { let name = node ["name"] . as_str () . unwrap_or ("unknown") ; let complexity = node ["complexity"] . as_f64 () . unwrap_or (0.0) ; let depth = node ["depth"] . as_u64 () . unwrap_or (0) as u32 ; let query = LMFDBQuery :: from_k_node (7 , complexity , depth , name) ; let llm_call = query . generate_llm_reflect_call ("k7.1") ; println ! ("🔍 K7.1 → LMFDB Query:") ; println ! ("📊 Collection: {}" , query . collection) ; println ! ("🌐 URL: {}" , query . to_lmfdb_url ()) ; println ! ("🎯 Features: {:?}" , query . similarity_features) ; println ! ("\n🤖 LLM Reflect Call:") ; println ! ("{}" , llm_call) ; } Ok (()) }