// Generated macro for from_derive_input (function)
macro_rules! Depcrate_derivefrom_derive_input {
() => {
// Module: crate::derive
// Provides: {"from_derive_input"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromDeriveInput` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_derive_input (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FdiOptions :: new (input)) }
};
}
