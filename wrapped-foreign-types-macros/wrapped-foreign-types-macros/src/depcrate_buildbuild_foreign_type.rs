// Generated macro for build_foreign_type (function)
macro_rules! Depcrate_buildbuild_foreign_type {
() => {
// Module: crate::build
// Provides: {"build_foreign_type"}
// Dependencies: {}
fn build_foreign_type (crate_ : & Path , input : & ForeignType) -> TokenStream { let decls = build_decls (crate_ , input) ; let oibits = build_oibits (crate_ , input) ; let foreign_impls = build_foreign_impls (crate_ , input) ; let drop_impl = build_drop_impl (crate_ , input) ; let deref_impls = build_deref_impls (crate_ , input) ; let borrow_impls = build_borrow_impls (crate_ , input) ; let as_ref_impls = build_as_ref_impls (crate_ , input) ; let clone_impl = build_clone_impl (crate_ , input) ; let to_owned_impl = build_to_owned_impl (crate_ , input) ; quote ! { # decls # oibits # foreign_impls # drop_impl # deref_impls # borrow_impls # as_ref_impls # clone_impl # to_owned_impl } }
};
}
