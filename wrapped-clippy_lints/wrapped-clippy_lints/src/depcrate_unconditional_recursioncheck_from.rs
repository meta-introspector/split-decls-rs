// Generated macro for check_from (function)
macro_rules! Depcrate_unconditional_recursioncheck_from {
() => {
// Module: crate::unconditional_recursion
// Provides: {"check_from"}
// Dependencies: {}
fn check_from (cx : & LateContext < '_ > , method_span : Span , method_def_id : LocalDefId , expr : & Expr < '_ >) { let Some (sig) = cx . typeck_results () . liberated_fn_sigs () . get (cx . tcx . local_def_id_to_hir_id (method_def_id)) else { return ; } ; if let Some ((fn_def_id , node_args)) = fn_def_id_with_node_args (cx , expr) && let [s1 , s2] = * * node_args && let (Some (s1) , Some (s2)) = (s1 . as_type () , s2 . as_type ()) && let Some (trait_def_id) = cx . tcx . trait_of_assoc (fn_def_id) && cx . tcx . is_diagnostic_item (sym :: Into , trait_def_id) && get_impl_trait_def_id (cx , method_def_id) == cx . tcx . get_diagnostic_item (sym :: From) && s1 == sig . inputs () [0] && s2 == sig . output () { span_error (cx , method_span , expr) ; } }
};
}
