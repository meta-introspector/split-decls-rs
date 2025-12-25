use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use cargo_lock::Lockfile;
use std::str::FromStr;

#[proc_macro]
pub fn import_cargo_lock(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let cargo_lock_path = path.value();
    
    // Read and parse Cargo.lock
    let lock_content = std::fs::read_to_string(&cargo_lock_path)
        .unwrap_or_else(|e| panic!("Failed to read Cargo.lock at {}: {}", cargo_lock_path, e));
    
    let lockfile = Lockfile::from_str(&lock_content)
        .unwrap_or_else(|e| panic!("Failed to parse Cargo.lock: {}", e));
    
    // Extract package info - only get packages with local paths
    let mut packages = Vec::new();
    for package in lockfile.packages {
        // Only include packages with local paths (workspace members)
        if let Some(source) = &package.source {
            if source.is_path() {
                let name = package.name.as_str();
                // Get the path from the source URL
                let path_str = source.url().path();
                packages.push(quote! {
                    CrateInfo { name: #name, path: #path_str }
                });
            }
        } else {
            // No source means it's a workspace member at root - use the package name as path
            let name = package.name.as_str();
            packages.push(quote! {
                CrateInfo { name: #name, path: #name }
            });
        }
    }
    
    let expanded = quote! {
        {
            #[derive(Debug, Clone)]
            pub struct CrateInfo {
                pub name: &'static str,
                pub path: &'static str,
            }
            
            const CARGO_LOCK_DATA: &[CrateInfo] = &[
                #(#packages),*
            ];
            
            CARGO_LOCK_DATA
        }
    };
    
    TokenStream::from(expanded)
}
