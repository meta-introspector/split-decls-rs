// Generated from: ./src/bin/proof_wrap_bin.rs
// Original file: ./src/bin/proof_wrap_bin.rs
// Function: exercise_wrapped_debug_file

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

#[decl_split_decls_rs_proof_wrap_bin]
# [doc = " Exercise wrapped DebugFile enum (simulated)  "] fn exercise_wrapped_debug_file () -> String { # [derive (Debug)] enum DebugFile { Primary , Supplementary , Dwo } let file = DebugFile :: Primary ; format ! ("DebugFile::{:?} variant used successfully" , file) }