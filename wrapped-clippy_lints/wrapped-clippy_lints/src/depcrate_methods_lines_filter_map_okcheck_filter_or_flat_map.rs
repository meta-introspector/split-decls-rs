// Generated macro for check_filter_or_flat_map (function)
macro_rules! Depcrate_methods_lines_filter_map_okcheck_filter_or_flat_map {
() => {
// Module: crate::methods::lines_filter_map_ok
// Provides: {"check_filter_or_flat_map"}
// Dependencies: {}
pub (super) fn check_filter_or_flat_map (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , method_name : & 'static str , method_arg : & Expr < '_ > , call_span : Span , msrv : Msrv ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && cx . typeck_results () . expr_ty_adjusted (recv) . is_diag_item (cx , sym :: IoLines) && match method_arg . kind { ExprKind :: Path (ref qpath) => cx . qpath_res (qpath , method_arg . hir_id) . is_diag_item (cx , sym :: result_ok_method) , ExprKind :: Closure (& Closure { body , .. }) => { if let Body { params : [param] , value , .. } = cx . tcx . hir_body (body) && let ExprKind :: MethodCall (method , receiver , [] , _) = value . kind { method . ident . name == sym :: ok && receiver . res_local_id () == Some (param . pat . hir_id) && cx . ty_based_def (* value) . is_diag_item (cx , sym :: result_ok_method) } else { false } } , _ => false , } && msrv . meets (cx , msrvs :: MAP_WHILE) { emit (cx , recv , method_name , call_span) ; } }
};
}
