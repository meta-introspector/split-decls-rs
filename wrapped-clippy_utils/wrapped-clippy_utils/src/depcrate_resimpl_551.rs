// Generated macro for impl_551 (impl)
macro_rules! Depcrate_resimpl_551 {
() => {
// Module: crate::res
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'tcx > MaybeQPath < 'tcx > for & 'tcx PatExpr < '_ > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { match & self . kind { PatExprKind :: Path (qpath) => Some ((qpath , self . hir_id)) , _ => None , } } }
};
}
