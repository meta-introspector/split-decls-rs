// Generated macro for check_if_try_match (function)
macro_rules! Depcrate_question_markcheck_if_try_match {
() => {
// Module: crate::question_mark
// Provides: {"check_if_try_match"}
// Dependencies: {}
fn check_if_try_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: Match (scrutinee , [arm1 , arm2] , MatchSource :: Normal | MatchSource :: Postfix) = expr . kind && ! expr . span . from_expansion () && let Some (mode) = find_try_mode (cx , scrutinee) && ! span_contains_cfg (cx , expr . span) && check_arms_are_try (cx , mode , arm1 , arm2) { let mut applicability = Applicability :: MachineApplicable ; let snippet = snippet_with_applicability (cx , scrutinee . span . source_callsite () , ".." , & mut applicability) ; span_lint_and_sugg (cx , QUESTION_MARK , expr . span , "this `match` expression can be replaced with `?`" , "try instead" , snippet . into_owned () + "?" , applicability ,) ; } }
};
}
