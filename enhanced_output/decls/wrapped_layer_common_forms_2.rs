// Generated from: ./src/bin/layer_common_forms.rs
// Original file: ./src/bin/layer_common_forms.rs
// Function: extract_common_form

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

#[decl_split_decls_rs_layer_common_forms]
fn extract_common_form (pattern : & str) -> String { if pattern . chars () . all (| c | c . is_ascii () && (c . is_alphabetic () || "🦄🔮🌟🎨🎪🐉💎🎭🦋" . contains (c))) { pattern . to_string () } else { let parts : Vec < & str > = pattern . split ('/') . collect () ; if parts . len () >= 3 { let first = parts . get (0) . unwrap_or (& "") ; let last = parts . last () . unwrap_or (& "") ; if parts . len () > 5 { format ! ("{}/.../{}/.../{}" , first , parts . get (parts . len () / 2) . unwrap_or (& "*") , last) } else if parts . len () > 3 { format ! ("{}/.../{}" , first , last) } else { format ! ("{}/{}/{}" , parts [0] , "*" , parts [parts . len () - 1]) } } else { pattern . replace (char :: is_numeric , "*") } } }