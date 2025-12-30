// Generated macro for check_struct (function)
macro_rules! Depcrate_check_checkcheck_struct {
() => {
// Module: crate::check::check
// Provides: {"check_struct"}
// Dependencies: {}
fn check_struct (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let def = tcx . adt_def (def_id) ; let span = tcx . def_span (def_id) ; def . destructor (tcx) ; if def . repr () . simd () { check_simd (tcx , span , def_id) ; } check_transparent (tcx , def) ; check_packed (tcx , span , def) ; }
};
}
