// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: print_similarity_report

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

#[decl_split_decls_rs_deepen_scanner]
fn print_similarity_report (similarities : & [Similarity]) { println ! ("\n🎯 Similarity Analysis Results:") ; println ! ("═══════════════════════════════") ; if similarities . is_empty () { println ! ("✅ No significant similarities found") ; return ; } for sim in similarities { println ! ("📊 {:.1}% similarity" , sim . score * 100.0) ; println ! ("   📼 Tape: {}" , sim . tape_macro) ; println ! ("   📂 Block: {}" , sim . output2_block) ; println ! () ; } }