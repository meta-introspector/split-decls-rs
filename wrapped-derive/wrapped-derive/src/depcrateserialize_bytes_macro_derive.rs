// Generated macro for serialize_bytes_macro_derive (function)
macro_rules! Depcrateserialize_bytes_macro_derive {
() => {
// Module: crate
// Provides: {"serialize_bytes_macro_derive"}
// Dependencies: {}
# [proc_macro_derive (TlsSerializeBytes , attributes (tls_codec))] pub fn serialize_bytes_macro_derive (input : TokenStream) -> TokenStream { let ast = parse_macro_input ! (input as DeriveInput) ; let parsed_ast = match parse_ast (ast) { Ok (ast) => ast , Err (err_ts) => return err_ts . into_compile_error () . into () , } ; impl_serialize (parsed_ast , SerializeVariant :: Bytes) . into () }
};
}
