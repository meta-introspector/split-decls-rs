// Generated macro for saturating_impl (macro)
macro_rules! Depcrate_ops_saturatingsaturating_impl {
() => {
// Module: crate::ops::saturating
// Provides: {"saturating_impl"}
// Dependencies: {}
macro_rules ! saturating_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self , v : & Self) -> Self { <$ t >::$ method (* self , * v) } } } ; }
};
}
