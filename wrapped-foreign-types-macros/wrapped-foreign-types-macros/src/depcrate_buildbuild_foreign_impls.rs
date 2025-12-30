// Generated macro for build_foreign_impls (function)
macro_rules! Depcrate_buildbuild_foreign_impls {
() => {
// Module: crate::build
// Provides: {"build_foreign_impls"}
// Dependencies: {}
fn build_foreign_impls (crate_ : & Path , input : & ForeignType) -> TokenStream { let name = & input . name ; let ctype = & input . ctype ; let ref_name = ref_name (input) ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; let phantom_data = input . phantom_data . as_ref () . map (| _ | quote ! (, # crate_ :: export :: PhantomData)) ; quote ! { unsafe impl # impl_generics # crate_ :: ForeignType for # name # ty_generics { type CType = # ctype ; type Ref = # ref_name # ty_generics ; # [inline] unsafe fn from_ptr (ptr : * mut # ctype) -> # name # ty_generics { debug_assert ! (! ptr . is_null ()) ; # name (<# crate_ :: export :: NonNull < _ >>:: new_unchecked (ptr) # phantom_data) } # [inline] fn as_ptr (& self) -> * mut # ctype { <# crate_ :: export :: NonNull < _ >>:: as_ptr (self . 0) } } unsafe impl # impl_generics # crate_ :: ForeignTypeRef for # ref_name # ty_generics { type CType = # ctype ; } } }
};
}
