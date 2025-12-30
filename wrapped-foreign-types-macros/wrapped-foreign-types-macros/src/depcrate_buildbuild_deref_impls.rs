// Generated macro for build_deref_impls (function)
macro_rules! Depcrate_buildbuild_deref_impls {
() => {
// Module: crate::build
// Provides: {"build_deref_impls"}
// Dependencies: {}
fn build_deref_impls (crate_ : & Path , input : & ForeignType) -> TokenStream { let name = & input . name ; let ref_name = ref_name (input) ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { impl # impl_generics # crate_ :: export :: Deref for # name # ty_generics { type Target = # ref_name # ty_generics ; # [inline] fn deref (& self) -> &# ref_name # ty_generics { unsafe { # crate_ :: ForeignTypeRef :: from_ptr (# crate_ :: ForeignType :: as_ptr (self)) } } } impl # impl_generics # crate_ :: export :: DerefMut for # name # ty_generics { # [inline] fn deref_mut (& mut self) -> & mut # ref_name # ty_generics { unsafe { # crate_ :: ForeignTypeRef :: from_ptr_mut (# crate_ :: ForeignType :: as_ptr (self)) } } } } }
};
}
