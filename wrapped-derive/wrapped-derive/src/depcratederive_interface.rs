// Generated macro for derive_interface (function)
macro_rules! Depcratederive_interface {
() => {
// Module: crate
// Provides: {"derive_interface"}
// Dependencies: {}
# [proc_macro_derive (Interface , attributes (graphql))] pub fn derive_interface (input : TokenStream) -> TokenStream { let interface_args = match args :: Interface :: from_derive_input (& parse_macro_input ! (input as DeriveInput)) { Ok (interface_args) => interface_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } ; match interface :: generate (& interface_args) { Ok (expanded) => expanded , Err (err) => err . write_errors () . into () , } }
};
}
