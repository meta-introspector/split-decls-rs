// Generated macro for impl_9429 (impl)
macro_rules! Depcrate_set_contains_or_insertimpl_9429 {
() => {
// Module: crate::set_contains_or_insert
// Provides: {"impl_9429"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SetContainsOrInsert { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! expr . span . from_expansion () && let Some (higher :: If { cond : cond_expr , then : then_expr , .. }) = higher :: If :: hir (expr) && let Some ((contains_expr , sym)) = try_parse_op_call (cx , cond_expr , sym :: contains) && let Some (insert_expr) = find_insert_calls (cx , & contains_expr , then_expr) { span_lint (cx , SET_CONTAINS_OR_INSERT , vec ! [contains_expr . span , insert_expr . span] , format ! ("usage of `{sym}::insert` after `{sym}::contains`") ,) ; } } }
};
}
