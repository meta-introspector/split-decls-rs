// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_redundant_pattern_matchcheck_if_let {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"check_if_let"}
// Dependencies: {}
pub (super) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , pat : & 'tcx Pat < '_ > , scrutinee : & 'tcx Expr < '_ > , has_else : bool , let_span : Span ,) { find_if_let_true (cx , pat , scrutinee , let_span) ; find_method_sugg_for_if_let (cx , expr , pat , scrutinee , "if" , has_else) ; }
};
}
