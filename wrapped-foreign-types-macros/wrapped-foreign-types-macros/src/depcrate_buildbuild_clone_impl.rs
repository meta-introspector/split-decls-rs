// Generated macro for build_clone_impl (function)
macro_rules! Depcrate_buildbuild_clone_impl {
() => {
// Module: crate::build
// Provides: {"build_clone_impl"}
// Dependencies: {}
fn build_clone_impl (crate_ : & Path , input : & ForeignType) -> TokenStream { let clone = match & input . clone { Some (clone) => clone , None => return quote ! () , } ; let name = & input . name ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { impl # impl_generics # crate_ :: export :: Clone for # name # ty_generics { # [inline] fn clone (& self) -> # name # ty_generics { unsafe { let ptr = (# clone) (# crate_ :: ForeignType :: as_ptr (self)) ; # crate_ :: ForeignType :: from_ptr (ptr) } } } } }
};
}
