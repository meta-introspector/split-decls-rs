// Generated macro for wrapping_impl (macro)
macro_rules! Depcrate_ops_wrappingwrapping_impl {
() => {
// Module: crate::ops::wrapping
// Provides: {"wrapping_impl"}
// Dependencies: {}
macro_rules ! wrapping_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self , v : & Self) -> Self { <$ t >::$ method (* self , * v) } } } ; ($ trait_name : ident , $ method : ident , $ t : ty , $ rhs : ty) => { impl $ trait_name <$ rhs > for $ t { # [inline] fn $ method (& self , v : &$ rhs) -> Self { <$ t >::$ method (* self , * v) } } } ; }
};
}
