// Generated macro for from_attributes (function)
macro_rules! Depcrate_derivefrom_attributes {
() => {
// Module: crate::derive
// Provides: {"from_attributes"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromAttributes` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_attributes (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FromAttributesOptions :: new (input)) }
};
}
