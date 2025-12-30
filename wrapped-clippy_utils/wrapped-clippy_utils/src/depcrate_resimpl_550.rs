// Generated macro for impl_550 (impl)
macro_rules! Depcrate_resimpl_550 {
() => {
// Module: crate::res
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'tcx > MaybeQPath < 'tcx > for & 'tcx Expr < '_ > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { match & self . kind { ExprKind :: Path (qpath) => Some ((qpath , self . hir_id)) , _ => None , } } }
};
}
