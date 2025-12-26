// Generated from: ./src/bin/macro_analyzer.rs
// Original file: ./src/bin/macro_analyzer.rs
// Function: calculate_and_format_scores

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

#[decl_split_decls_rs_macro_analyzer]
fn calculate_and_format_scores (analysis : TermAnalysis , term_scores_by_macro : HashMap < String , HashMap < Term , f64 > > , global_module_term_scores : HashMap < Term , (f64 , f64) > ,) -> MacroAnalysisOutput { let mut output_module_frequencies = HashMap :: new () ; for (term , count) in analysis . module_frequencies { output_module_frequencies . insert (term . to_string () , count) ; } let mut output_global_frequencies = HashMap :: new () ; for (term , count) in analysis . global_frequencies { output_global_frequencies . insert (term . to_string () , count) ; } let mut output_term_scores_by_macro = HashMap :: new () ; for (macro_name , term_map) in term_scores_by_macro { let mut inner_map = HashMap :: new () ; for (term , score) in term_map { inner_map . insert (term . to_string () , score) ; } output_term_scores_by_macro . insert (macro_name , inner_map) ; } let mut output_global_module_term_scores = HashMap :: new () ; for (term , scores) in global_module_term_scores { output_global_module_term_scores . insert (term . to_string () , scores) ; } MacroAnalysisOutput { total_module_terms : analysis . total_module_terms , total_global_terms : analysis . total_global_terms , module_frequencies : output_module_frequencies , global_frequencies : output_global_frequencies , term_scores_by_macro : output_term_scores_by_macro , global_module_term_scores : output_global_module_term_scores , } }