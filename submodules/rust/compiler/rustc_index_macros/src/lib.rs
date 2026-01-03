#![feature(rustc_attrs)]

#![feature(type_alias_impl_trait)]
// Stub proc macros for rustc_index
use proc_macro::TokenStream;

#[proc_macro]
pub fn newtype_index(_input: TokenStream) -> TokenStream {
    // Simple stub - just return empty
    TokenStream::new()
}
