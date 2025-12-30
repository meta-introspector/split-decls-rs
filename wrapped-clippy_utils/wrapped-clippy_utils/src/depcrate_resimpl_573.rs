// Generated macro for impl_573 (impl)
macro_rules! Depcrate_resimpl_573 {
() => {
// Module: crate::res
// Provides: {"impl_573"}
// Dependencies: {}
impl < T : MaybeDef > MaybeDef for Binder < '_ , T > { # [inline] fn opt_def_id (self) -> Option < DefId > { self . skip_binder () . opt_def_id () } # [inline] fn opt_def < 'tcx > (self , tcx : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { self . skip_binder () . opt_def (tcx) } }
};
}
