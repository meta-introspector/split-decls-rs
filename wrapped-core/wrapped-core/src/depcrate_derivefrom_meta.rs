// Generated macro for from_meta (function)
macro_rules! Depcrate_derivefrom_meta {
() => {
// Module: crate::derive
// Provides: {"from_meta"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromMeta` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_meta (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FromMetaOptions :: new (input)) }
};
}
