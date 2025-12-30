// Generated macro for impl_391 (impl)
macro_rules! Depcrate_scalar_valueimpl_391 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_391"}
// Dependencies: {}
impl Field { # [doc = " Returns a [`Field`] for constructing or matching over a [`Variant`]."] fn match_arg (& self) -> TokenStream { match self { Self :: Named (_) => quote ! { { # self : v } } , Self :: Unnamed => quote ! { (v) } , } } }
};
}
