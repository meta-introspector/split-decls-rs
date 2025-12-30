// Generated macro for impl_566 (impl)
macro_rules! Depcrate_resimpl_566 {
() => {
// Module: crate::res
// Provides: {"impl_566"}
// Dependencies: {}
impl MaybeDef for DefId { # [inline] fn opt_def_id (self) -> Option < DefId > { Some (self) } # [inline] fn opt_def < 'tcx > (self , tcx : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { self . opt_def_id () . map (| id | (tcx . tcx () . def_kind (id) , id)) } }
};
}
