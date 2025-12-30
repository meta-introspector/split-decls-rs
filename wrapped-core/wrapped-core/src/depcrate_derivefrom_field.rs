// Generated macro for from_field (function)
macro_rules! Depcrate_derivefrom_field {
() => {
// Module: crate::derive
// Provides: {"from_field"}
// Dependencies: {}
# [doc = " Create tokens for a `darling::FromField` impl from a `DeriveInput`. If"] # [doc = " the input cannot produce a valid impl, the returned tokens will contain"] # [doc = " compile errors instead."] pub fn from_field (input : & DeriveInput) -> TokenStream { emit_impl_or_error ! (options :: FromFieldOptions :: new (input)) }
};
}
