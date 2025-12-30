// Generated macro for find_if_let_true (function)
macro_rules! Depcrate_matches_redundant_pattern_matchfind_if_let_true {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"find_if_let_true"}
// Dependencies: {}
# [doc = " Looks for any of:"] # [doc = " * `if let true = ...`"] # [doc = " * `if let false = ...`"] # [doc = " * `while let true = ...`"] fn find_if_let_true < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ > , scrutinee : & 'tcx Expr < '_ > , let_span : Span) { find_match_true (cx , pat , scrutinee , let_span , "using `if let` to pattern match a bool") ; }
};
}
