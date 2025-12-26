// Generated from: ./src/bin/comprehensive_ngram_analyzer.rs
// Original file: ./src/bin/comprehensive_ngram_analyzer.rs
// Function: analyze_layer_ngrams

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

#[decl_split_decls_rs_comprehensive_ngram_analyzer]
fn analyze_layer_ngrams (translation : & serde_json :: Value , layer : u8) -> Result < LayerNGramAnalysis > { let compressions = translation ["layer_compressions"] . as_array () . unwrap () ; let layer_data = & compressions [layer as usize] ; let input_patterns = layer_data ["input_patterns"] . as_array () . unwrap () ; let output_patterns = layer_data ["output_emojis"] . as_array () . unwrap () ; let ngram_2 = extract_top_ngrams (input_patterns , 2) ; let ngram_3 = extract_top_ngrams (input_patterns , 3) ; let ngram_5 = extract_top_ngrams (input_patterns , 5) ; let ngram_7 = extract_top_ngrams (input_patterns , 7) ; Ok (LayerNGramAnalysis { layer , input_size : input_patterns . len () , output_size : output_patterns . len () , ngram_2 , ngram_3 , ngram_5 , ngram_7 , }) }