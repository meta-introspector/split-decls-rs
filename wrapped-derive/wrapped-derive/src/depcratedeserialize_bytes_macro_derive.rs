// Generated macro for deserialize_bytes_macro_derive (function)
macro_rules! Depcratedeserialize_bytes_macro_derive {
() => {
// Module: crate
// Provides: {"deserialize_bytes_macro_derive"}
// Dependencies: {}
# [proc_macro_derive (TlsDeserializeBytes , attributes (tls_codec))] pub fn deserialize_bytes_macro_derive (input : TokenStream) -> TokenStream { let ast = parse_macro_input ! (input as DeriveInput) ; let parsed_ast = match parse_ast (ast) { Ok (ast) => ast , Err (err_ts) => return err_ts . into_compile_error () . into () , } ; impl_deserialize_bytes (parsed_ast) . into () }
};
}
