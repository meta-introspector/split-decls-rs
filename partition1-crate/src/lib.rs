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
            
            // Import real implementations from wrapped crates
            include!("../../output2/wrapped-mockall_derive/src/decls/demutify_arg.rs");
            include!("../../output2/wrapped-mockall_derive/src/decls/compile_error.rs");
            include!("../../output2/wrapped-mockall_derive/src/decls/pat_is_self.rs");
            
            // Import syn types and functions
            use quote::*;
            use syn::{
                FnArg, Generics, Ident, Pat, PatType, Signature, 
                Token, Type, TypeParamBound, WherePredicate,
                GenericParam, parse2
            };
            use syn::punctuated::Punctuated;
            use proc_macro2::TokenStream;
            use std::collections::HashMap;
            
            // Import real types from wrapped crates instead of stubs
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
