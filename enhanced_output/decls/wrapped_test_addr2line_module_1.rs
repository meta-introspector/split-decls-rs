// Generated from: ./src/bin/test_addr2line_module.rs
// Original file: ./src/bin/test_addr2line_module.rs
// Function: test_wrapped_addr2line_items

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

#[decl_split_decls_rs_test_addr2line_module]
# [doc = " Exercise the wrapped addr2line items directly"] fn test_wrapped_addr2line_items () -> Result < Vec < String > > { let mut results = Vec :: new () ; let _error : Error = "test error" . to_string () ; results . push ("Error type - created and used successfully" . to_string ()) ; let primary = DebugFile :: Primary ; let supplementary = DebugFile :: Supplementary ; let dwo = DebugFile :: Dwo ; assert_eq ! (primary , DebugFile :: Primary) ; assert_ne ! (primary , supplementary) ; results . push ("DebugFile enum - all variants created and compared" . to_string ()) ; let attrs = RangeAttributes :: default () ; let _attrs2 = RangeAttributes { } ; results . push ("RangeAttributes struct - created with default and custom" . to_string ()) ; match primary { DebugFile :: Primary => results . push ("Pattern matching - Primary variant matched" . to_string ()) , DebugFile :: Supplementary => return Err (anyhow :: anyhow ! ("Wrong variant")) , DebugFile :: Dwo => return Err (anyhow :: anyhow ! ("Wrong variant")) , } let debug_str = format ! ("{:?}" , primary) ; assert ! (debug_str . contains ("Primary")) ; results . push ("Debug formatting - DebugFile formats correctly" . to_string ()) ; let cloned = primary . clone () ; assert_eq ! (primary , cloned) ; results . push ("Cloning - DebugFile clones correctly" . to_string ()) ; Ok (results) }