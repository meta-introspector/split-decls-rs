// Generated macro for lint_uncovered_ty_params (function)
macro_rules! Depcrate_coherence_orphanlint_uncovered_ty_params {
() => {
// Module: crate::coherence::orphan
// Provides: {"lint_uncovered_ty_params"}
// Dependencies: {}
fn lint_uncovered_ty_params < 'tcx > (tcx : TyCtxt < 'tcx > , UncoveredTyParams { uncovered , local_ty } : UncoveredTyParams < TyCtxt < 'tcx > , FxIndexSet < DefId > > , impl_def_id : LocalDefId ,) { let hir_id = tcx . local_def_id_to_hir_id (impl_def_id) ; for param_def_id in uncovered { let span = tcx . def_ident_span (param_def_id) . unwrap () ; let name = tcx . item_ident (param_def_id) ; match local_ty { Some (local_type) => tcx . emit_node_span_lint (UNCOVERED_PARAM_IN_PROJECTION , hir_id , span , errors :: TyParamFirstLocalLint { span , note : () , param : name , local_type } ,) , None => tcx . emit_node_span_lint (UNCOVERED_PARAM_IN_PROJECTION , hir_id , span , errors :: TyParamSomeLint { span , note : () , param : name } ,) , } ; } }
};
}
