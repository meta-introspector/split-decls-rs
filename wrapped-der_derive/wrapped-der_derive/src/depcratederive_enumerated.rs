// Generated macro for derive_enumerated (function)
macro_rules! Depcratederive_enumerated {
() => {
// Module: crate
// Provides: {"derive_enumerated"}
// Dependencies: {}
# [doc = " Derive decoders and encoders for ASN.1 [`Enumerated`] types on a"] # [doc = " C-like `enum` type."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " The `Enumerated` proc macro requires a C-like enum which impls `Copy`"] # [doc = " and has a `#[repr]` of `u8`, `u16`, or `u32`:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use der::Enumerated;"] # [doc = ""] # [doc = " #[derive(Enumerated, Copy, Clone, Debug, Eq, PartialEq)]"] # [doc = " #[repr(u32)]"] # [doc = " pub enum CrlReason {"] # [doc = "     Unspecified = 0,"] # [doc = "     KeyCompromise = 1,"] # [doc = "     CaCompromise = 2,"] # [doc = "     AffiliationChanged = 3,"] # [doc = "     Superseded = 4,"] # [doc = "     CessationOfOperation = 5,"] # [doc = "     CertificateHold = 6,"] # [doc = "     RemoveFromCrl = 8,"] # [doc = "     PrivilegeWithdrawn = 9,"] # [doc = "     AaCompromised = 10"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that the derive macro will write a `TryFrom<...>` impl for the"] # [doc = " provided `#[repr]`, which is used by the decoder."] # [proc_macro_derive (Enumerated , attributes (asn1))] pub fn derive_enumerated (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; match DeriveEnumerated :: new (input) { Ok (t) => t . to_tokens () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
