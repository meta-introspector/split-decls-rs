// Generated macro for from_variant (function)
macro_rules! Depcrate_derivefrom_variant {
() => {
// Module: crate::derive
// Provides: {"from_variant"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromVariant` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_variant (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FromVariantOptions :: new (input)) }
};
}
