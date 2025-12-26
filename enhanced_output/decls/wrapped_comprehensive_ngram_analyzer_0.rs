// Generated from: ./src/bin/comprehensive_ngram_analyzer.rs
// Original file: ./src/bin/comprehensive_ngram_analyzer.rs
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

#[decl_split_decls_rs_comprehensive_ngram_analyzer]
fn main () -> Result < () > { println ! ("🔍 COMPREHENSIVE N-GRAM ANALYSIS (2,3,5,7-grams)") ; println ! ("================================================") ; let translation_data = fs :: read_to_string ("output2_emoji_translation.json") ? ; let translation : serde_json :: Value = serde_json :: from_str (& translation_data) ? ; let mut all_analyses = Vec :: new () ; for layer in 0 .. 8 { println ! ("\n📊 Analyzing Layer {} N-grams..." , layer) ; let analysis = analyze_layer_ngrams (& translation , layer) ? ; println ! ("   Input: {} tokens" , analysis . input_size) ; println ! ("   Output: {} tokens" , analysis . output_size) ; println ! ("   2-grams: {}, 3-grams: {}, 5-grams: {}, 7-grams: {}" , analysis . ngram_2 . len () , analysis . ngram_3 . len () , analysis . ngram_5 . len () , analysis . ngram_7 . len ()) ; all_analyses . push (analysis) ; } generate_comprehensive_report (& all_analyses) ? ; let analysis_json = serde_json :: to_string_pretty (& all_analyses) ? ; fs :: write ("comprehensive_ngram_analysis.json" , analysis_json) ? ; println ! ("\n🎯 COMPREHENSIVE N-GRAM ANALYSIS COMPLETE") ; println ! ("========================================") ; println ! ("✅ All 8 layers analyzed") ; println ! ("✅ 2,3,5,7-grams extracted per layer") ; println ! ("✅ Top 10 patterns identified for each n-gram size") ; println ! ("✅ Pattern types classified") ; Ok (()) }