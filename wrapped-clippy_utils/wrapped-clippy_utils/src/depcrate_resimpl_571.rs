// Generated macro for impl_571 (impl)
macro_rules! Depcrate_resimpl_571 {
() => {
// Module: crate::res
// Provides: {"impl_571"}
// Dependencies: {}
impl < T : MaybeDef > MaybeDef for Option < T > { # [inline] fn opt_def_id (self) -> Option < DefId > { self . and_then (T :: opt_def_id) } # [inline] fn opt_def < 'tcx > (self , tcx : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { self . and_then (| x | T :: opt_def (x , tcx)) } }
};
}
