// Generated from: ./src/bin/recursive_compressor.rs
// Original file: ./src/bin/recursive_compressor.rs
// Function: compress_layer

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
fn compress_layer (input_vocab : & HashMap < String , String > , layer_num : u8) -> Result < LayerVocab > { let mut compressed = HashMap :: new () ; let mut pattern_counts : HashMap < String , usize > = HashMap :: new () ; for pattern in input_vocab . values () { let fragments : Vec < & str > = pattern . split (& ['(' , ')' , ',' , ' ']) . collect () ; for fragment in fragments { if ! fragment . is_empty () && fragment . len () > 2 { * pattern_counts . entry (fragment . to_string ()) . or_insert (0) += 1 ; } } } let mut sorted_fragments : Vec < _ > = pattern_counts . into_iter () . collect () ; sorted_fragments . sort_by (| a , b | b . 1 . cmp (& a . 1)) ; let emojis = ["🔥" , "⚡" , "🎯" , "🚀" , "💎" , "🌟" , "🔮" , "🎨" , "🎪" , "🎭" , "🦄" , "🐉" , "🦋" , "🌸" , "🍀" , "🎲" , "💫" , "✨" , "🌈" , "🎨" , "🎯" , "🔥" , "💎" , "🌟"] ; for (i , (fragment , count)) in sorted_fragments . iter () . take (100) . enumerate () { if count > & 2 { let emoji_idx = (i + layer_num as usize * 100) % emojis . len () ; let emoji = emojis [emoji_idx] ; compressed . insert (emoji . to_string () , fragment . clone ()) ; } } let compression_ratio = compressed . len () as f64 / input_vocab . len () as f64 ; Ok (LayerVocab { layer : layer_num , vocab : compressed , compression_ratio , parent_layer : if layer_num > 0 { Some (layer_num - 1) } else { None } , }) }