// Generated macro for incoherent_inherent_impl_crates (function)
macro_rules! Depcrate_consteval_tests_method_resolutionincoherent_inherent_impl_crates {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"incoherent_inherent_impl_crates"}
// Dependencies: {}
pub (crate) fn incoherent_inherent_impl_crates (db : & dyn HirDatabase , krate : Crate , fp : TyFingerprint ,) -> SmallVec < [Crate ; 2] > { let _p = tracing :: info_span ! ("incoherent_inherent_impl_crates") . entered () ; let mut res = SmallVec :: new () ; for krate in db . transitive_deps (krate) { let impls = db . inherent_impls_in_crate (krate) ; if impls . map . get (& fp) . is_some_and (| v | ! v . is_empty ()) { res . push (krate) ; } } res }
};
}
