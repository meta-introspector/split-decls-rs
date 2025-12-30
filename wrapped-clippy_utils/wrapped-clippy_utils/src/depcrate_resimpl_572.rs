// Generated macro for impl_572 (impl)
macro_rules! Depcrate_resimpl_572 {
() => {
// Module: crate::res
// Provides: {"impl_572"}
// Dependencies: {}
impl < T : MaybeDef > MaybeDef for EarlyBinder < '_ , T > { # [inline] fn opt_def_id (self) -> Option < DefId > { self . skip_binder () . opt_def_id () } # [inline] fn opt_def < 'tcx > (self , tcx : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { self . skip_binder () . opt_def (tcx) } }
};
}
