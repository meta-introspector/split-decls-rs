// Generated macro for build_as_ref_impls (function)
macro_rules! Depcrate_buildbuild_as_ref_impls {
() => {
// Module: crate::build
// Provides: {"build_as_ref_impls"}
// Dependencies: {}
fn build_as_ref_impls (crate_ : & Path , input : & ForeignType) -> TokenStream { let name = & input . name ; let ref_name = ref_name (input) ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { impl # impl_generics # crate_ :: export :: AsRef <# ref_name # ty_generics > for # name # ty_generics { # [inline] fn as_ref (& self) -> &# ref_name # ty_generics { &** self } } impl # impl_generics # crate_ :: export :: AsMut <# ref_name # ty_generics > for # name # ty_generics { # [inline] fn as_mut (& mut self) -> & mut # ref_name # ty_generics { & mut ** self } } } }
};
}
