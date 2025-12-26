// Generated from: ./src/bin/output2_emoji_translator.rs
// Original file: ./src/bin/output2_emoji_translator.rs
// Function: create_reconstruction_proof

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

#[decl_split_decls_rs_output2_emoji_translator]
fn create_reconstruction_proof (files : & [String] , emojis : & [String]) -> Result < ReconstructionProof > { let mut emoji_to_files = HashMap :: new () ; for (i , file) in files . iter () . enumerate () { let emoji_idx = i % emojis . len () ; let emoji = & emojis [emoji_idx] ; emoji_to_files . entry (emoji . clone ()) . or_insert_with (Vec :: new) . push (file . clone ()) ; } let compression_ratio = files . len () as f64 / emojis . len () as f64 ; Ok (ReconstructionProof { emoji_to_files , total_files_encoded : files . len () , compression_achieved : compression_ratio , }) }