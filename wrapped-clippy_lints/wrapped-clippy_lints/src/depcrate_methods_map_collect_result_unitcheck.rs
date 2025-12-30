// Generated macro for check (function)
macro_rules! Depcrate_methods_map_collect_result_unitcheck {
() => {
// Module: crate::methods::map_collect_result_unit
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , iter : & hir :: Expr < '_ > , map_fn : & hir :: Expr < '_ >) { let collect_ret_ty = cx . typeck_results () . expr_ty (expr) ; if collect_ret_ty . is_diag_item (cx , sym :: Result) && let ty :: Adt (_ , args) = collect_ret_ty . kind () && let Some (result_t) = args . types () . next () && result_t . is_unit () { span_lint_and_sugg (cx , MAP_COLLECT_RESULT_UNIT , expr . span , "`.map().collect()` can be replaced with `.try_for_each()`" , "try" , format ! ("{}.try_for_each({})" , snippet (cx , iter . span , "..") , snippet (cx , map_fn . span , "..")) , Applicability :: MachineApplicable ,) ; } }
};
}
