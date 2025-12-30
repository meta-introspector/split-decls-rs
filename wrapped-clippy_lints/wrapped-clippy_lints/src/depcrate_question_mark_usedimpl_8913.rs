// Generated macro for impl_8913 (impl)
macro_rules! Depcrate_question_mark_usedimpl_8913 {
() => {
// Module: crate::question_mark_used
// Provides: {"impl_8913"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for QuestionMarkUsed { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Match (_ , _ , MatchSource :: TryDesugar (_)) = expr . kind { if ! span_is_local (expr . span) { return ; } # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , QUESTION_MARK_USED , expr . span , "the `?` operator was used" , | diag | { diag . help ("consider using a custom macro or match expression") ; }) ; } } }
};
}
