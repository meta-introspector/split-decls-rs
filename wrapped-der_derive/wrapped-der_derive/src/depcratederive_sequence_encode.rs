// Generated macro for derive_sequence_encode (function)
macro_rules! Depcratederive_sequence_encode {
() => {
// Module: crate
// Provides: {"derive_sequence_encode"}
// Dependencies: {}
# [doc = " Derive the [`EncodeValue`][1] trait on a `struct`."] # [doc = ""] # [doc = " [1]: https://docs.rs/der/latest/der/trait.EncodeValue.html"] # [proc_macro_derive (EncodeValue , attributes (asn1))] pub fn derive_sequence_encode (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; match DeriveSequence :: new (input) { Ok (t) => t . to_tokens_encode () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
