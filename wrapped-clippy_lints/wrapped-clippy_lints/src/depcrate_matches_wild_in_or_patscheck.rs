// Generated macro for check (function)
macro_rules! Depcrate_matches_wild_in_or_patscheck {
() => {
// Module: crate::matches::wild_in_or_pats
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , arms : & [Arm < '_ >]) { let ty = cx . typeck_results () . expr_ty (expr) . peel_refs () ; if let ty :: Adt (adt_def , _) = ty . kind () && has_non_exhaustive_attr (cx . tcx , * adt_def) { return ; } for arm in arms { if let PatKind :: Or (fields) = arm . pat . kind && fields . len () > 1 && fields . iter () . any (is_wild) { span_lint_and_help (cx , WILDCARD_IN_OR_PATTERNS , arm . pat . span , "wildcard pattern covers any other pattern as it will match anyway" , None , "consider handling `_` separately" ,) ; } } }
};
}
