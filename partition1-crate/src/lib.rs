// Partition 1 crate - contains 9 nodes from output2

// Import the attribute macros
use llm_macros::{llm_error_message, llm_context};

// Simplest possible macros - now define the functions
macro_rules! mkdeclfn { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! mkdeclimpl { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! mkdeclstruct { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! compare_tail { ($($tt:tt)*) => { false }; }

macro_rules! partition_module {
    ($mod_name:ident, $file_path:expr) => {
        pub mod $mod_name {
            // Import the attribute macros into this module
            use llm_macros::{llm_error_message, llm_context};
            
            // Only add what's not already imported by the modules
            use quote::*;
            use syn::{
                FnArg, Generics, Ident, Pat, PatType, Signature, 
                Token, Type, TypeParamBound, WherePredicate,
                GenericParam, parse2
            };
            use syn::punctuated::Punctuated;
            use proc_macro2::TokenStream;
            
            // Define missing functions
            fn compile_error(_span: proc_macro2::Span, _msg: &str) {}
            fn demutify_arg(_arg: &mut PatType) {}
            
            // Stub out attribute macros by defining them as empty
            use proc_macro2::Span;
            
            // Define missing types with Hash and Eq for HashMap usage
            #[derive(Hash, Eq, PartialEq, Clone)]
            pub struct Bom;
            pub struct DeclContext;
            pub struct DeclOrigin;
            pub trait HirDatabase {}
            
            // Now include the actual file content
            include!(concat!("/home/mdupont/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/", $file_path));
        }
        pub use $mod_name::*;
    };
}

include!("../../partition_1_single.rs");
