// Generated macro for TokenStream (struct)
macro_rules! DepcrateTokenStream {
() => {
// Module: crate
// Provides: {"TokenStream"}
// Dependencies: {}
# [doc = " The main type provided by this crate, representing an abstract stream of"] # [doc = " tokens, or, more specifically, a sequence of token trees."] # [doc = " The type provides interfaces for iterating over those token trees and, conversely,"] # [doc = " collecting a number of token trees into one stream."] # [doc = ""] # [doc = " This is both the input and output of `#[proc_macro]`, `#[proc_macro_attribute]`"] # [doc = " and `#[proc_macro_derive]` definitions."] # [cfg_attr (feature = "rustc-dep-of-std" , rustc_diagnostic_item = "TokenStream")] # [stable (feature = "proc_macro_lib" , since = "1.15.0")] # [derive (Clone)] pub struct TokenStream (Option < bridge :: client :: TokenStream >) ;
};
}
