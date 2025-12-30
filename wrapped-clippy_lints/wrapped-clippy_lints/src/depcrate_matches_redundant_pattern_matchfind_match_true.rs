// Generated macro for find_match_true (function)
macro_rules! Depcrate_matches_redundant_pattern_matchfind_match_true {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"find_match_true"}
// Dependencies: {}
# [doc = " Common logic between `find_if_let_true` and `check_matches_true`"] fn find_match_true < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ > , scrutinee : & 'tcx Expr < '_ > , span : Span , message : & 'static str ,) { if let PatKind :: Expr (lit) = pat . kind && let PatExprKind :: Lit { lit , negated : false } = lit . kind && let LitKind :: Bool (pat_is_true) = lit . node { let mut applicability = Applicability :: MachineApplicable ; let mut sugg = Sugg :: hir_with_context (cx , scrutinee , scrutinee . span . source_callsite () . ctxt () , ".." , & mut applicability ,) ; if ! pat_is_true { sugg = make_unop ("!" , sugg) ; } span_lint_and_sugg (cx , REDUNDANT_PATTERN_MATCHING , span , message , "consider using the condition directly" , sugg . into_string () , applicability ,) ; } }
};
}
