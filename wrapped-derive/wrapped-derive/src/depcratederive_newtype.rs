// Generated macro for derive_newtype (function)
macro_rules! Depcratederive_newtype {
() => {
// Module: crate
// Provides: {"derive_newtype"}
// Dependencies: {}
# [proc_macro_derive (NewType , attributes (graphql))] pub fn derive_newtype (input : TokenStream) -> TokenStream { let newtype_args = match args :: NewType :: from_derive_input (& parse_macro_input ! (input as DeriveInput)) { Ok (newtype_args) => newtype_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } ; match newtype :: generate (& newtype_args) { Ok (expanded) => expanded , Err (err) => err . write_errors () . into () , } }
};
}
