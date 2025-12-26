// Generated from: ./src/bin/output2_emoji_translator.rs
// Original file: ./src/bin/output2_emoji_translator.rs
// Function: apply_8_layer_compression

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
fn apply_8_layer_compression (files : & [String]) -> Result < Vec < LayerCompression > > { let mut compressions = Vec :: new () ; let mut current_patterns = files . to_vec () ; for layer in 0 .. 8 { let compression_factor = match layer { 0 => 0.8 , 1 => 0.7 , 2 => 0.6 , 3 => 0.5 , 4 => 0.4 , 5 => 0.3 , 6 => 0.2 , 7 => 0.1 , _ => 1.0 , } ; let target_size = (current_patterns . len () as f64 * compression_factor) . max (1.0) as usize ; let compressed = compress_to_emojis (& current_patterns , target_size , layer) ? ; let ratio = current_patterns . len () as f64 / compressed . len () as f64 ; compressions . push (LayerCompression { layer , input_patterns : current_patterns . clone () , output_emojis : compressed . clone () , compression_ratio : ratio , }) ; println ! ("   Layer {}: {} → {} patterns ({:.2}x)" , layer , current_patterns . len () , compressed . len () , ratio) ; current_patterns = compressed ; } Ok (compressions) }