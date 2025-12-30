// Generated macro for check (function)
macro_rules! Depcrate_methods_zst_offsetcheck {
() => {
// Module: crate::methods::zst_offset
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ >) { if let ty :: RawPtr (ty , _) = cx . typeck_results () . expr_ty (recv) . kind () && let Ok (layout) = cx . tcx . layout_of (cx . typing_env () . as_query_input (* ty)) && layout . is_zst () { span_lint (cx , ZST_OFFSET , expr . span , "offset calculation on zero-sized value") ; } }
};
}
