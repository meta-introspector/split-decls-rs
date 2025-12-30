// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_manual_filtercheck_if_let {
() => {
// Module: crate::matches::manual_filter
// Provides: {"check_if_let"}
// Dependencies: {}
pub (super) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , let_pat : & 'tcx Pat < '_ > , let_expr : & 'tcx Expr < '_ > , then_expr : & 'tcx Expr < '_ > , else_expr : & 'tcx Expr < '_ > ,) { check (cx , expr , let_expr , let_pat , then_expr , None , else_expr) ; }
};
}
