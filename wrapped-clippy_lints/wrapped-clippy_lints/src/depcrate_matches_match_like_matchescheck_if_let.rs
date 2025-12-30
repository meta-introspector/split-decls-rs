// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_match_like_matchescheck_if_let {
() => {
// Module: crate::matches::match_like_matches
// Provides: {"check_if_let"}
// Dependencies: {}
pub (crate) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , let_pat : & 'tcx Pat < '_ > , let_expr : & 'tcx Expr < '_ > , then_expr : & 'tcx Expr < '_ > , else_expr : & 'tcx Expr < '_ > ,) { if ! span_contains_comment (cx . sess () . source_map () , expr . span) && cx . typeck_results () . expr_ty (expr) . is_bool () && let Some (b0) = find_bool_lit (then_expr) && let Some (b1) = find_bool_lit (else_expr) && b0 != b1 { if ! is_lint_allowed (cx , REDUNDANT_PATTERN_MATCHING , let_pat . hir_id) && is_some_wild (let_pat . kind) { return ; } let mut applicability = Applicability :: MaybeIncorrect ; let pat = snippet_with_applicability (cx , let_pat . span , ".." , & mut applicability) ; let mut ex_new = let_expr ; if let ExprKind :: AddrOf (BorrowKind :: Ref , .. , ex_inner) = let_expr . kind && let ty :: Ref (..) = cx . typeck_results () . expr_ty (ex_inner) . kind () { ex_new = ex_inner ; } span_lint_and_sugg (cx , MATCH_LIKE_MATCHES_MACRO , expr . span , "if let .. else expression looks like `matches!` macro" , "try" , format ! ("{}matches!({}, {pat})" , if b0 { "" } else { "!" } , snippet_with_applicability (cx , ex_new . span , ".." , & mut applicability) ,) , applicability ,) ; } }
};
}
