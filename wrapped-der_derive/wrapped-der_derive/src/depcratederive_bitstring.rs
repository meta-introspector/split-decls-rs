// Generated macro for derive_bitstring (function)
macro_rules! Depcratederive_bitstring {
() => {
// Module: crate
// Provides: {"derive_bitstring"}
// Dependencies: {}
# [doc = " Derive the [`BitString`] on a `struct` with bool fields."] # [doc = ""] # [doc = " ```ignore"] # [doc = " use der::BitString;"] # [doc = ""] # [doc = " #[derive(BitString)]"] # [doc = " pub struct MyFlags {"] # [doc = "     pub flag_0: bool,"] # [doc = "     pub flag_1: bool,"] # [doc = "     pub flag_2: bool,"] # [doc = " }"] # [doc = " ```"] # [proc_macro_derive (BitString , attributes (asn1))] pub fn derive_bitstring (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; match DeriveBitString :: new (input) { Ok (t) => t . to_tokens () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
