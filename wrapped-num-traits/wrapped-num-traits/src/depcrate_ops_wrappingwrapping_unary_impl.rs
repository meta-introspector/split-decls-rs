// Generated macro for wrapping_unary_impl (macro)
macro_rules! Depcrate_ops_wrappingwrapping_unary_impl {
() => {
// Module: crate::ops::wrapping
// Provides: {"wrapping_unary_impl"}
// Dependencies: {}
macro_rules ! wrapping_unary_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self) -> $ t { <$ t >::$ method (* self) } } } ; }
};
}
