// Generated macro for impl_9799 (impl)
macro_rules! Depcrate_single_call_fnimpl_9799 {
() => {
// Module: crate::single_call_fn
// Provides: {"impl_9799"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SingleCallFn { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Path (qpath) = expr . kind && let res = cx . qpath_res (& qpath , expr . hir_id) && let Some (call_def_id) = res . opt_def_id () && let Some (def_id) = call_def_id . as_local () && let DefKind :: Fn | DefKind :: AssocFn = cx . tcx . def_kind (def_id) && is_valid_item_kind (cx , def_id) { match self . def_id_to_usage . entry (def_id) { IndexEntry :: Occupied (mut entry) => { if let CallState :: Once { .. } = entry . get () { entry . insert (CallState :: Multiple) ; } } , IndexEntry :: Vacant (entry) => { entry . insert (CallState :: Once { call_site : expr . span }) ; } , } } } fn check_crate_post (& mut self , cx : & LateContext < 'tcx >) { for (& def_id , usage) in & self . def_id_to_usage { if let CallState :: Once { call_site } = * usage && let fn_hir_id = cx . tcx . local_def_id_to_hir_id (def_id) && let fn_span = cx . tcx . hir_span_with_body (fn_hir_id) && ! self . is_function_allowed (cx , def_id , fn_hir_id , fn_span) { span_lint_hir_and_then (cx , SINGLE_CALL_FN , fn_hir_id , fn_span , "this function is only used once" , | diag | { diag . span_note (call_site , "used here") ; } ,) ; } } } }
};
}
