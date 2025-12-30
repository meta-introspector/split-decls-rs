// Generated macro for check (function)
macro_rules! Depcrate_loops_char_indices_as_byte_indicescheck {
() => {
// Module: crate::loops::char_indices_as_byte_indices
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , pat : & Pat < '_ > , iterable : & Expr < '_ > , body : & 'tcx Expr < 'tcx >) { if let ExprKind :: MethodCall (_ , enumerate_recv , _ , enumerate_span) = iterable . kind && let Some (method_id) = cx . typeck_results () . type_dependent_def_id (iterable . hir_id) && cx . tcx . is_diagnostic_item (sym :: enumerate_method , method_id) && let ExprKind :: MethodCall (_ , chars_recv , _ , chars_span) = enumerate_recv . kind && let Some (method_id) = cx . typeck_results () . type_dependent_def_id (enumerate_recv . hir_id) && cx . tcx . is_diagnostic_item (sym :: str_chars , method_id) { if let PatKind :: Tuple ([pat , _] , _) = pat . kind && let PatKind :: Binding (_ , binding_id , ..) = pat . kind { for_each_expr (cx , body , | expr | { if expr . res_local_id () == Some (binding_id) { check_index_usage (cx , expr , pat , enumerate_span , chars_span , chars_recv) ; } CONTINUE }) ; } else if let PatKind :: Binding (_ , binding_id , ..) = pat . kind { for_each_expr (cx , body , | expr | { if let ExprKind :: Field (e , field) = expr . kind && e . res_local_id () == Some (binding_id) && field . name == sym :: integer (0) { check_index_usage (cx , expr , pat , enumerate_span , chars_span , chars_recv) ; } CONTINUE }) ; } } }
};
}
