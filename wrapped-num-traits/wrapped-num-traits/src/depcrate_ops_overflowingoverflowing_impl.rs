// Generated macro for overflowing_impl (macro)
macro_rules! Depcrate_ops_overflowingoverflowing_impl {
() => {
// Module: crate::ops::overflowing
// Provides: {"overflowing_impl"}
// Dependencies: {}
macro_rules ! overflowing_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self , v : & Self) -> (Self , bool) { <$ t >::$ method (* self , * v) } } } ; }
};
}
