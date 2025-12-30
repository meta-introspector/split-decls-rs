// Generated macro for derive_value_ord (function)
macro_rules! Depcratederive_value_ord {
() => {
// Module: crate
// Provides: {"derive_value_ord"}
// Dependencies: {}
# [doc = " Derive the [`ValueOrd`][1] trait on a `struct`."] # [doc = ""] # [doc = " This trait is used in conjunction with ASN.1 `SET OF` types to determine"] # [doc = " the lexicographical order of their DER encodings."] # [doc = ""] # [doc = " [1]: https://docs.rs/der/latest/der/trait.ValueOrd.html"] # [proc_macro_derive (ValueOrd , attributes (asn1))] pub fn derive_value_ord (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; match DeriveValueOrd :: new (input) { Ok (t) => t . to_tokens () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
