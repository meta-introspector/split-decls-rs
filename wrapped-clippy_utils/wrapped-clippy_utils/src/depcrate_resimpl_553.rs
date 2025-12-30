// Generated macro for impl_553 (impl)
macro_rules! Depcrate_resimpl_553 {
() => {
// Module: crate::res
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'tcx > MaybeQPath < 'tcx > for & '_ Pat < 'tcx > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { match self . kind { PatKind :: Expr (e) => e . opt_qpath () , _ => None , } } }
};
}
