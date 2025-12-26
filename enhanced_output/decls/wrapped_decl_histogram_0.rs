// Generated from: ./src/bin/decl_histogram.rs
// Original file: ./src/bin/decl_histogram.rs
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

#[decl_split_decls_rs_decl_histogram]
fn main () -> Result < () , Box < dyn std :: error :: Error > > { let base_path = PathBuf :: from ("../../") ; let mut histogram : HashMap < String , u32 > = HashMap :: new () ; let mut total_files = 0 ; println ! ("📊 Declaration Type Histogram Report") ; println ! ("=====================================") ; find_decl_files (& base_path , & mut histogram , & mut total_files) ? ; let mut sorted : Vec < _ > = histogram . iter () . collect () ; sorted . sort_by (| a , b | b . 1 . cmp (a . 1)) ; println ! ("\n📈 Declaration Types Found:") ; println ! ("{:<20} {:<8} {}" , "Type" , "Count" , "Bar") ; println ! ("{:-<50}" , "") ; let max_count = sorted . first () . map (| (_ , count) | * * count) . unwrap_or (0) ; for (decl_type , count) in sorted { let bar_length = if max_count > 0 { (* count * 40 / max_count) . max (1) } else { 0 } ; let bar = "█" . repeat (bar_length as usize) ; println ! ("{:<20} {:<8} {}" , decl_type , count , bar) ; } println ! ("\n📋 Summary:") ; println ! ("Total declaration files: {}" , total_files) ; println ! ("Unique declaration types: {}" , histogram . len ()) ; Ok (()) }