// Generated macro for variances (function)
macro_rules! Depcrate_variance_dumpvariances {
() => {
// Module: crate::variance::dump
// Provides: {"variances"}
// Dependencies: {}
pub (crate) fn variances (tcx : TyCtxt < '_ >) { let crate_items = tcx . hir_crate_items (()) ; if tcx . has_attr (CRATE_DEF_ID , sym :: rustc_variance_of_opaques) { for id in crate_items . opaques () { tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id) , variances : format_variances (tcx , id) , }) ; } } for id in crate_items . free_items () { if ! tcx . has_attr (id . owner_id , sym :: rustc_variance) { continue ; } tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id . owner_id) , variances : format_variances (tcx , id . owner_id . def_id) , }) ; } }
};
}
