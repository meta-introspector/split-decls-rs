// Generated macro for impl_570 (impl)
macro_rules! Depcrate_resimpl_570 {
() => {
// Module: crate::res
// Provides: {"impl_570"}
// Dependencies: {}
impl MaybeDef for Res { # [inline] fn opt_def_id (self) -> Option < DefId > { Res :: opt_def_id (& self) } # [inline] fn opt_def < 'tcx > (self , _ : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { match self { Res :: Def (kind , id) => Some ((kind , id)) , _ => None , } } }
};
}
