// Generated macro for check (function)
macro_rules! Depcrate_matches_redundant_pattern_matchcheck {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some (higher :: WhileLet { let_pat , let_expr , let_span , .. }) = higher :: WhileLet :: hir (expr) { find_method_sugg_for_if_let (cx , expr , let_pat , let_expr , "while" , false) ; find_if_let_true (cx , let_pat , let_expr , let_span) ; } }
};
}
