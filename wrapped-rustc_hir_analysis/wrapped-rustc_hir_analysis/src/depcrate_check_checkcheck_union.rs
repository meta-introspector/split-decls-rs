// Generated macro for check_union (function)
macro_rules! Depcrate_check_checkcheck_union {
() => {
// Module: crate::check::check
// Provides: {"check_union"}
// Dependencies: {}
fn check_union (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let def = tcx . adt_def (def_id) ; let span = tcx . def_span (def_id) ; def . destructor (tcx) ; check_transparent (tcx , def) ; check_union_fields (tcx , span , def_id) ; check_packed (tcx , span , def) ; }
};
}
