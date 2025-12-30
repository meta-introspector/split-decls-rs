// Generated macro for impl_552 (impl)
macro_rules! Depcrate_resimpl_552 {
() => {
// Module: crate::res
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'tcx , AmbigArg > MaybeQPath < 'tcx > for & 'tcx hir :: Ty < '_ , AmbigArg > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { match & self . kind { TyKind :: Path (qpath) => Some ((qpath , self . hir_id)) , _ => None , } } }
};
}
