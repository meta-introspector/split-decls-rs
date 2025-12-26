// Generated from: ./src/bin/recursive_compressor.rs
// Original file: ./src/bin/recursive_compressor.rs
// Function: recursive_compress

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

#[decl_split_decls_rs_recursive_compressor]
fn recursive_compress (initial_vocab : HashMap < String , String >) -> Result < RecursiveCompression > { let mut layers = Vec :: new () ; let mut current_vocab = initial_vocab . clone () ; let original_size = initial_vocab . len () ; println ! ("🔄 RECURSIVE COMPRESSION - 8 Layers Deep") ; println ! ("========================================") ; for layer in 0 .. 8 { println ! ("\n📊 Layer {}: Processing {} patterns" , layer , current_vocab . len ()) ; let layer_vocab = compress_layer (& current_vocab , layer) ? ; println ! ("   Compressed to {} emoji tokens (ratio: {:.2})" , layer_vocab . vocab . len () , layer_vocab . compression_ratio) ; let mut next_vocab = HashMap :: new () ; for (emoji , pattern) in & current_vocab { let compressed_pattern = apply_compression (pattern , & layer_vocab . vocab) ; if compressed_pattern != * pattern { next_vocab . insert (emoji . clone () , compressed_pattern) ; } } let samples : Vec < _ > = layer_vocab . vocab . iter () . take (5) . collect () ; for (emoji , pattern) in samples { println ! ("   {} = {}" , emoji , pattern) ; } layers . push (layer_vocab) ; current_vocab = next_vocab ; if current_vocab . is_empty () { println ! ("   🎯 Compression complete at layer {}" , layer) ; break ; } } let final_size = layers . last () . map (| l | l . vocab . len ()) . unwrap_or (0) ; let total_compression = final_size as f64 / original_size as f64 ; Ok (RecursiveCompression { layers , total_compression , original_patterns : original_size , final_vocab_size : final_size , }) }