// Generated macro for build_drop_impl (function)
macro_rules! Depcrate_buildbuild_drop_impl {
() => {
// Module: crate::build
// Provides: {"build_drop_impl"}
// Dependencies: {}
fn build_drop_impl (crate_ : & Path , input : & ForeignType) -> TokenStream { let name = & input . name ; let drop = & input . drop ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { impl # impl_generics # crate_ :: export :: Drop for # name # ty_generics { # [inline] fn drop (& mut self) { unsafe { (# drop) (# crate_ :: ForeignType :: as_ptr (self)) ; } } } } }
};
}
