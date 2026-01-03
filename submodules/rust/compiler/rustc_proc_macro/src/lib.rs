// Stub implementation of rustc_proc_macro to avoid standard library proc_macro issues
// This replaces the problematic path = "../../library/proc_macro/src/lib.rs"

#![allow(unused)]

// Create minimal stubs for proc_macro types instead of importing
// since proc_macro is a special built-in crate

pub struct TokenStream;
pub struct Group;
pub struct Ident;
pub struct Punct;
pub struct Literal;
pub struct Span;

impl TokenStream {
    pub fn new() -> Self { TokenStream }
}

// Additional stubs for any rustc-specific proc_macro functionality
pub mod bridge {
    pub struct Client;
    pub struct Server;
}

pub mod diagnostic {
    pub struct Diagnostic;
}
