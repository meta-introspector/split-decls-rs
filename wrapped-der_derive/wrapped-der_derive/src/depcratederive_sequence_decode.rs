// Generated macro for derive_sequence_decode (function)
macro_rules! Depcratederive_sequence_decode {
() => {
// Module: crate
// Provides: {"derive_sequence_decode"}
// Dependencies: {}
# [doc = " Derive the [`DecodeValue`][1] trait on a `struct`."] # [doc = ""] # [doc = " [1]: https://docs.rs/der/latest/der/trait.DecodeValue.html"] # [proc_macro_derive (DecodeValue , attributes (asn1))] pub fn derive_sequence_decode (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; match DeriveSequence :: new (input) { Ok (t) => t . to_tokens_decode () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
