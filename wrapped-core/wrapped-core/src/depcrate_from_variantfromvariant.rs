// Generated macro for FromVariant (trait)
macro_rules! Depcrate_from_variantFromVariant {
() => {
// Module: crate::from_variant
// Provides: {"FromVariant"}
// Dependencies: {}
# [doc = " Creates an instance from a specified `syn::Variant`."] pub trait FromVariant : Sized { # [doc = " Create an instance from `syn::Variant`, or return an error."] fn from_variant (variant : & Variant) -> Result < Self > ; }
};
}
