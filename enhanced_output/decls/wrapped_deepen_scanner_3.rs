// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: find_similarities

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
fn find_similarities (tape : & HashMap < String , String > , blocks : & HashMap < String , String >) -> Result < Vec < Similarity > > { let mut similarities = Vec :: new () ; for (tape_name , tape_content) in tape { for (block_name , block_content) in blocks { let similarity_score = calculate_similarity (tape_content , block_content) ; if similarity_score > 0.3 { similarities . push (Similarity { tape_macro : tape_name . clone () , output2_block : block_name . clone () , score : similarity_score , }) ; } } } similarities . sort_by (| a , b | b . score . partial_cmp (& a . score) . unwrap ()) ; Ok (similarities) }