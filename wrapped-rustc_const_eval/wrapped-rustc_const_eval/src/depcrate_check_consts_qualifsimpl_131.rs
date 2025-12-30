// Generated macro for impl_131 (impl)
macro_rules! Depcrate_check_consts_qualifsimpl_131 {
() => {
// Module: crate::check_consts::qualifs
// Provides: {"impl_131"}
// Dependencies: {}
impl Qualif for NeedsDrop { const ANALYSIS_NAME : & 'static str = "flow_needs_drop" ; const IS_CLEARED_ON_MOVE : bool = true ; const ALLOW_PROMOTED : bool = true ; fn in_qualifs (qualifs : & ConstQualifs) -> bool { qualifs . needs_drop } fn in_any_value_of_ty < 'tcx > (cx : & ConstCx < '_ , 'tcx > , ty : Ty < 'tcx >) -> bool { ty . needs_drop (cx . tcx , cx . typing_env) } fn is_structural_in_adt_value < 'tcx > (cx : & ConstCx < '_ , 'tcx > , adt : AdtDef < 'tcx >) -> bool { ! adt . has_dtor (cx . tcx) } }
};
}
