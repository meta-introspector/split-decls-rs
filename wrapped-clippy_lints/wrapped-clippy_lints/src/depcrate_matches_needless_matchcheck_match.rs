// Generated macro for check_match (function)
macro_rules! Depcrate_matches_needless_matchcheck_match {
() => {
// Module: crate::matches::needless_match
// Provides: {"check_match"}
// Dependencies: {}
pub (crate) fn check_match (cx : & LateContext < '_ > , ex : & Expr < '_ > , arms : & [Arm < '_ >] , expr : & Expr < '_ >) { if arms . len () > 1 && expr_ty_matches_p_ty (cx , ex , expr) && check_all_arms (cx , ex , arms) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , NEEDLESS_MATCH , expr . span , "this match expression is unnecessary" , "replace it with" , snippet_with_applicability (cx , ex . span , ".." , & mut applicability) . to_string () , applicability ,) ; } }
};
}
