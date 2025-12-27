extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Syscall attribute macro that wraps function calls with safety checks
#[proc_macro_attribute]
pub fn syscall(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let syscall_type = if !args.is_empty() {
        args.to_string().trim_matches('"').to_string()
    } else {
        "unknown".to_string()
    };
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            // Syscall safety wrapper
            eprintln!("AUDIT: {} called syscall type: {}", stringify!(#fn_name), #syscall_type);
            
            // Original function body
            #fn_block
        }
    };
    
    TokenStream::from(expanded)
}

/// Declarative macro to handle inline syscall syntax
#[proc_macro]
pub fn syscall_inline(input: TokenStream) -> TokenStream {
    // Parse the input to extract syscall type and expression
    let input_str = input.to_string();
    
    // Handle different syscall types
    if input_str.contains("\"read\"") {
        let expr = input_str.split("\"read\"").nth(1).unwrap_or("").trim();
        let expanded = quote! {
            {
                eprintln!("SYSCALL_AUDIT: read operation");
                #expr
            }
        };
        TokenStream::from(expanded)
    } else if input_str.contains("\"write\"") {
        let expr = input_str.split("\"write\"").nth(1).unwrap_or("").trim();
        let expanded = quote! {
            {
                eprintln!("SYSCALL_AUDIT: write operation");
                #expr
            }
        };
        TokenStream::from(expanded)
    } else if input_str.contains("\"exec\"") {
        let expr = input_str.split("\"exec\"").nth(1).unwrap_or("").trim();
        let expanded = quote! {
            {
                eprintln!("SYSCALL_AUDIT: exec operation");
                #expr
            }
        };
        TokenStream::from(expanded)
    } else {
        // Default passthrough
        input
    }
}

/// Expression-level macro for audited filesystem reads
#[proc_macro]
pub fn syscall_read(input: TokenStream) -> TokenStream {
    let input_tokens: TokenStream2 = input.into();
    let expanded = quote! {
        {
            eprintln!("SYSCALL_AUDIT: read operation");
            std::fs::read_dir(#input_tokens)
        }
    };
    TokenStream::from(expanded)
}

/// Expression-level macro for audited filesystem file reads
#[proc_macro]
pub fn syscall_read_file(input: TokenStream) -> TokenStream {
    let input_tokens: TokenStream2 = input.into();
    let expanded = quote! {
        {
            eprintln!("SYSCALL_AUDIT: read_file operation");
            std::fs::read(#input_tokens)
        }
    };
    TokenStream::from(expanded)
}

/// Expression-level macro for audited filesystem writes
#[proc_macro]
pub fn syscall_write(input: TokenStream) -> TokenStream {
    let input_tokens: TokenStream2 = input.into();
    let expanded = quote! {
        {
            eprintln!("SYSCALL_AUDIT: write operation");
            std::fs::write(#input_tokens)
        }
    };
    TokenStream::from(expanded)
}

/// Expression-level macro for audited process execution
#[proc_macro]
pub fn syscall_exec(input: TokenStream) -> TokenStream {
    let input_tokens: TokenStream2 = input.into();
    let expanded = quote! {
        {
            eprintln!("SYSCALL_AUDIT: exec operation");
            std::process::Command::new(#input_tokens)
        }
    };
    TokenStream::from(expanded)
}
