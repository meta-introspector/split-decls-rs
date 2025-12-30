// Generated macro for check_matches_true (function)
macro_rules! Depcrate_matches_redundant_pattern_matchcheck_matches_true {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"check_matches_true"}
// Dependencies: {}
# [doc = " Looks for:"] # [doc = " * `matches!(expr, true)`"] pub fn check_matches_true < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , arm : & 'tcx Arm < '_ > , scrutinee : & 'tcx Expr < '_ > ,) { find_match_true (cx , arm . pat , scrutinee , expr . span . source_callsite () , "using `matches!` to pattern match a bool" ,) ; }
};
}
