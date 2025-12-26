// Generated from: ./src/bin/proof_decl2addr.rs
// Original file: ./src/bin/proof_decl2addr.rs
// Function: export_decl_mapping

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

#[decl_split_decls_rs_proof_decl2addr]
# [doc = " Export declaration mapping as JSON"] fn export_decl_mapping (decls : & HashMap < String , DeclAddress >) -> Result < String > { let mut json = String :: from ("{\n") ; for (i , (name , decl_addr)) in decls . iter () . enumerate () { if i > 0 { json . push_str (",\n") ; } json . push_str (& format ! ("  \"{}\": {{\"addr\": \"{}\", \"type\": \"{}\", \"path\": \"{}\"}}" , name , decl_addr . address , decl_addr . decl_type , decl_addr . source_path)) ; } json . push_str ("\n}") ; Ok (json) }