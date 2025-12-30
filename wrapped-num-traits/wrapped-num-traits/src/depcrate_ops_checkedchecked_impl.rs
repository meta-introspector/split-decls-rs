// Generated macro for checked_impl (macro)
macro_rules! Depcrate_ops_checkedchecked_impl {
() => {
// Module: crate::ops::checked
// Provides: {"checked_impl"}
// Dependencies: {}
macro_rules ! checked_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self , v : &$ t) -> Option <$ t > { <$ t >::$ method (* self , * v) } } } ; }
};
}
