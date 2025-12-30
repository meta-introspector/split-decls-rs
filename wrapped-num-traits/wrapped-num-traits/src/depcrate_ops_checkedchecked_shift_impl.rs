// Generated macro for checked_shift_impl (macro)
macro_rules! Depcrate_ops_checkedchecked_shift_impl {
() => {
// Module: crate::ops::checked
// Provides: {"checked_shift_impl"}
// Dependencies: {}
macro_rules ! checked_shift_impl { ($ trait_name : ident , $ method : ident , $ t : ty) => { impl $ trait_name for $ t { # [inline] fn $ method (& self , rhs : u32) -> Option <$ t > { <$ t >::$ method (* self , rhs) } } } ; }
};
}
