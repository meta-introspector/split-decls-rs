use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Ident};

/// Replace a module/function with a generated wrapper
/// 
/// Usage:
/// ```rust
/// use_wrapper!(original_module::function_name => "path/to/generated/wrapper");
/// ```
#[proc_macro]
pub fn use_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as UseWrapperInput);
    
    let original_path = &input.original_path;
    let wrapper_path = &input.wrapper_path;
    
    quote! {
        // Re-export the wrapper as the original name
        pub use #wrapper_path as #original_path;
    }.into()
}

/// Test a single wrapped module in isolation
/// 
/// Usage:
/// ```rust
/// test_wrapper!("path/to/generated/wrapper", original_tests);
/// ```
#[proc_macro]
pub fn test_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TestWrapperInput);
    
    let wrapper_path = &input.wrapper_path;
    let test_name = &input.test_name;
    
    quote! {
        #[cfg(test)]
        mod #test_name {
            use super::*;
            use #wrapper_path;
            
            #[test]
            fn test_wrapper_compiles() {
                // Basic compilation test
                assert!(true);
            }
            
            #[test] 
            fn test_wrapper_interface() {
                // Test that wrapper has expected interface
                // TODO: Add interface validation
            }
        }
    }.into()
}

/// Conditionally replace module based on feature flag
/// 
/// Usage:
/// ```rust
/// conditional_wrapper!(
///     feature = "use_wrapped_alloc",
///     original = std::alloc,
///     wrapper = "generated/alloc_wrapper"
/// );
/// ```
#[proc_macro]
pub fn conditional_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ConditionalWrapperInput);
    
    let feature = &input.feature;
    let original = &input.original;
    let wrapper = &input.wrapper;
    
    quote! {
        #[cfg(feature = #feature)]
        pub use #wrapper as #original;
        
        #[cfg(not(feature = #feature))]
        pub use #original;
    }.into()
}

// Parser structs
struct UseWrapperInput {
    original_path: syn::Path,
    wrapper_path: syn::Path,
}

struct TestWrapperInput {
    wrapper_path: syn::Path,
    test_name: Ident,
}

struct ConditionalWrapperInput {
    feature: LitStr,
    original: syn::Path,
    wrapper: syn::Path,
}

impl syn::parse::Parse for UseWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let original_path = input.parse()?;
        input.parse::<syn::Token![=>]>()?;
        let wrapper_path = input.parse()?;
        
        Ok(UseWrapperInput {
            original_path,
            wrapper_path,
        })
    }
}

impl syn::parse::Parse for TestWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrapper_path = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let test_name = input.parse()?;
        
        Ok(TestWrapperInput {
            wrapper_path,
            test_name,
        })
    }
}

impl syn::parse::Parse for ConditionalWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Ident>()?; // "feature"
        input.parse::<syn::Token![=]>()?;
        let feature = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        
        input.parse::<syn::Ident>()?; // "original"
        input.parse::<syn::Token![=]>()?;
        let original = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        
        input.parse::<syn::Ident>()?; // "wrapper"
        input.parse::<syn::Token![=]>()?;
        let wrapper = input.parse()?;
        
        Ok(ConditionalWrapperInput {
            feature,
            original,
            wrapper,
        })
    }
}
