// Generated macro for derive_description (function)
macro_rules! Depcratederive_description {
() => {
// Module: crate
// Provides: {"derive_description"}
// Dependencies: {}
# [proc_macro_derive (Description , attributes (graphql))] pub fn derive_description (input : TokenStream) -> TokenStream { let desc_args = match args :: Description :: from_derive_input (& parse_macro_input ! (input as DeriveInput)) { Ok (desc_args) => desc_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } ; match description :: generate (& desc_args) { Ok (expanded) => expanded , Err (err) => err . write_errors () . into () , } }
};
}
