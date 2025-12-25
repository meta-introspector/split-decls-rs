use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A procedural macro to find Rust compilers in the Nix store.
/// It can optionally take a string literal as a filter.
/// Returns a Vec<String> of paths.
///
/// Usage:
/// `let rustc_paths: Vec<String> = find_nix_rustc!();`
/// `let rustc_filtered_paths: Vec<String> = find_nix_rustc!("1.91");`
#[proc_macro]
#[decl(fn, name = "find_nix_rustc", vis = "pub", hash = "9ccd7a99")]
pub fn find_nix_rustc(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as macro_args::FindRustcArgs);
    let filter_str = args.filter.map(|lit| lit.value());
    let nix_store_path = Path::new("/nix/store");
    let mut rustc_paths = Vec::new();
    if let Ok(entries) = fs::read_dir(nix_store_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let rustc_candidate_path = path.join("bin").join("rustc");
                if rustc_candidate_path.exists() && rustc_candidate_path.is_file() {
                    if let Some(path_str) = rustc_candidate_path.to_str() {
                        if let Some(ref filter) = filter_str {
                            if path_str.contains(filter) {
                                rustc_paths.push(path_str.to_string());
                            }
                        } else {
                            rustc_paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }
    let quoted_paths = rustc_paths.iter().map(|s| {
        quote! {
            # s.to_string()
        }
    });
    let output = quote! {
        vec![# (# quoted_paths),*]
    };
    output.into()
}
