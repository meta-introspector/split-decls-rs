// Generated macro for build_oibit (function)
macro_rules! Depcrate_buildbuild_oibit {
() => {
// Module: crate::build
// Provides: {"build_oibit"}
// Dependencies: {}
fn build_oibit (crate_ : & Path , input : & ForeignType , oibit : & Ident) -> TokenStream { let name = & input . name ; let ref_name = ref_name (input) ; let (impl_generics , ty_generics , _) = input . generics . split_for_impl () ; quote ! { unsafe impl # impl_generics # crate_ :: export ::# oibit for # name # ty_generics { } unsafe impl # impl_generics # crate_ :: export ::# oibit for # ref_name # ty_generics { } } }
};
}
