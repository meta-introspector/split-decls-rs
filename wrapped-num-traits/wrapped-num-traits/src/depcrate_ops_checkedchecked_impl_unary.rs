// Generated macro for checked_impl_unary (macro)
macro_rules! Depcrate_ops_checkedchecked_impl_unary {
() => {
// Module: crate::ops::checked
// Provides: {"checked_impl_unary"}
// Dependencies: {}
macro_rules ! checked_impl_unary { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self) -> Option <$ t > { <$ t >::$ method (* self) } } } ; }
};
}
