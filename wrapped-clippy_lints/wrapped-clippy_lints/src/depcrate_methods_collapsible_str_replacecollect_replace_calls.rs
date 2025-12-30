// Generated macro for collect_replace_calls (function)
macro_rules! Depcrate_methods_collapsible_str_replacecollect_replace_calls {
() => {
// Module: crate::methods::collapsible_str_replace
// Provides: {"collect_replace_calls"}
// Dependencies: {}
fn collect_replace_calls < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , to_arg : & 'tcx hir :: Expr < 'tcx > ,) -> ReplaceMethods < 'tcx > { let mut methods = VecDeque :: new () ; let mut from_args = VecDeque :: new () ; let _ : Option < () > = for_each_expr_without_closures (expr , | e | { if let Some ((sym :: replace , _ , [from , to] , _ , _)) = method_call (e) { if eq_expr_value (cx , to_arg , to) && cx . typeck_results () . expr_ty (from) . peel_refs () . is_char () { methods . push_front (e) ; from_args . push_front (from) ; ControlFlow :: Continue (()) } else { ControlFlow :: Break (()) } } else { ControlFlow :: Continue (()) } }) ; ReplaceMethods { methods , from_args } }
};
}
