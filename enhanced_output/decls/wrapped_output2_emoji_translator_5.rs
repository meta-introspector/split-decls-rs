// Generated from: ./src/bin/output2_emoji_translator.rs
// Original file: ./src/bin/output2_emoji_translator.rs
// Function: reverse_translate_emojis

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
fn reverse_translate_emojis (emojis : & [String] , proof : & ReconstructionProof) -> Result < Vec < String > > { let mut reconstructed = Vec :: new () ; for emoji in emojis { if let Some (files) = proof . emoji_to_files . get (emoji) { reconstructed . extend (files . clone ()) ; } } Ok (reconstructed) }