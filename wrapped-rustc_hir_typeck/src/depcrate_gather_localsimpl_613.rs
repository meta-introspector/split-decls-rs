// Generated macro for impl_613 (impl)
macro_rules! Depcrate_gather_localsimpl_613 {
() => {
// Module: crate::gather_locals
// Provides: {"impl_613"}
// Dependencies: {}
impl < 'a > From < & 'a hir :: LetStmt < 'a > > for Declaration < 'a > { fn from (local : & 'a hir :: LetStmt < 'a >) -> Self { let hir :: LetStmt { hir_id , super_ : _ , pat , ty , span , init , els , source : _ } = * local ; Declaration { hir_id , pat , ty , span , init , origin : DeclOrigin :: LocalDecl { els } } } }
};
}
