// Generated macro for should_lint (function)
macro_rules! Depcrate_lines_filter_map_okshould_lint {
() => {
// Module: crate::lines_filter_map_ok
// Provides: {"should_lint"}
// Dependencies: {}
fn should_lint (cx : & LateContext < '_ > , args : & [Expr < '_ >] , method_name : Symbol) -> bool { match args { [] => method_name == sym :: flatten , [fm_arg] => { match & fm_arg . kind { ExprKind :: Path (qpath) => cx . qpath_res (qpath , fm_arg . hir_id) . opt_def_id () . is_some_and (| did | cx . tcx . is_diagnostic_item (sym :: result_ok_method , did)) , ExprKind :: Closure (Closure { body , .. }) => { if let Body { params : [param] , value , .. } = cx . tcx . hir_body (* body) && let ExprKind :: MethodCall (method , receiver , [] , _) = value . kind && path_to_local_id (receiver , param . pat . hir_id) && let Some (method_did) = cx . typeck_results () . type_dependent_def_id (value . hir_id) { is_diag_item_method (cx , method_did , sym :: Result) && method . ident . name == sym :: ok } else { false } } , _ => false , } } , _ => false , } }
};
}
