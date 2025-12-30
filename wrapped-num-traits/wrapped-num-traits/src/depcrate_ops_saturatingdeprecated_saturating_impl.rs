// Generated macro for deprecated_saturating_impl (macro)
macro_rules! Depcrate_ops_saturatingdeprecated_saturating_impl {
() => {
// Module: crate::ops::saturating
// Provides: {"deprecated_saturating_impl"}
// Dependencies: {}
macro_rules ! deprecated_saturating_impl { ($ trait_name : ident for $ ($ t : ty) *) => { $ (impl $ trait_name for $ t { # [inline] fn saturating_add (self , v : Self) -> Self { Self :: saturating_add (self , v) } # [inline] fn saturating_sub (self , v : Self) -> Self { Self :: saturating_sub (self , v) } }) * } }
};
}
