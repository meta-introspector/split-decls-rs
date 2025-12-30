// Generated macro for from_type_param (function)
macro_rules! Depcrate_derivefrom_type_param {
() => {
// Module: crate::derive
// Provides: {"from_type_param"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromTypeParam` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_type_param (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FromTypeParamOptions :: new (input)) }
};
}
