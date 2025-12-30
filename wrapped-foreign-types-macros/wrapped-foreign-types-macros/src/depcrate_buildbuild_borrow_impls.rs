// Generated macro for build_borrow_impls (function)
macro_rules! Depcrate_buildbuild_borrow_impls {
() => {
// Module: crate::build
// Provides: {"build_borrow_impls"}
// Dependencies: {}
fn build_borrow_impls (crate_ : & Path , input : & ForeignType) -> TokenStream { let name = & input . name ; let ref_name = ref_name (input) ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { impl # impl_generics # crate_ :: export :: Borrow <# ref_name # ty_generics > for # name # ty_generics { # [inline] fn borrow (& self) -> &# ref_name # ty_generics { &** self } } impl # impl_generics # crate_ :: export :: BorrowMut <# ref_name # ty_generics > for # name # ty_generics { # [inline] fn borrow_mut (& mut self) -> & mut # ref_name # ty_generics { & mut ** self } } } }
};
}
