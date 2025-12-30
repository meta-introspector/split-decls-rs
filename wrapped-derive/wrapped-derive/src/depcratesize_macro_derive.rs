// Generated macro for size_macro_derive (function)
macro_rules! Depcratesize_macro_derive {
() => {
// Module: crate
// Provides: {"size_macro_derive"}
// Dependencies: {}
# [proc_macro_derive (TlsSize , attributes (tls_codec))] pub fn size_macro_derive (input : TokenStream) -> TokenStream { let ast = parse_macro_input ! (input as DeriveInput) ; let parsed_ast = match parse_ast (ast) { Ok (ast) => ast , Err (err_ts) => return err_ts . into_compile_error () . into () , } ; impl_tls_size (parsed_ast) . into () }
};
}
