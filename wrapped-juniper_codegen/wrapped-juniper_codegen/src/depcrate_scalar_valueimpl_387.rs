// Generated macro for impl_387 (impl)
macro_rules! Depcrate_scalar_valueimpl_387 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_387"}
// Dependencies: {}
impl Variant { # [doc = " Returns generated code for matching over this [`Variant`]."] fn match_arm (& self) -> TokenStream { let (ident , field) = (& self . ident , & self . field . match_arg ()) ; quote ! { Self ::# ident # field } } }
};
}
