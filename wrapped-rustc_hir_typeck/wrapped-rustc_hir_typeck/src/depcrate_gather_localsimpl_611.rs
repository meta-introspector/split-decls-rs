// Generated macro for impl_611 (impl)
macro_rules! Depcrate_gather_localsimpl_611 {
() => {
// Module: crate::gather_locals
// Provides: {"impl_611"}
// Dependencies: {}
impl < 'a > DeclOrigin < 'a > { pub (super) fn try_get_else (& self) -> Option < & 'a hir :: Block < 'a > > { match self { Self :: LocalDecl { els } => * els , Self :: LetExpr => None , } } }
};
}
