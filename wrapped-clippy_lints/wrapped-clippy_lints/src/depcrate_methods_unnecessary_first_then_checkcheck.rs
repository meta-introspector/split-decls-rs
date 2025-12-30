// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_first_then_checkcheck {
() => {
// Module: crate::methods::unnecessary_first_then_check
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , call_span : Span , first_call : & Expr < '_ > , first_caller : & Expr < '_ > , is_some : bool ,) { if ! cx . typeck_results () . expr_ty_adjusted (first_caller) . peel_refs () . is_slice () { return ; } let ExprKind :: MethodCall (_ , _ , _ , first_call_span) = first_call . kind else { return ; } ; let both_calls_span = first_call_span . with_hi (call_span . hi ()) ; if let Some (both_calls_snippet) = both_calls_span . get_source_text (cx) && let Some (first_caller_snippet) = first_caller . span . get_source_text (cx) { let (sugg_span , suggestion) = if is_some { (first_caller . span . with_hi (call_span . hi ()) , format ! ("!{first_caller_snippet}.is_empty()") ,) } else { (both_calls_span , "is_empty()" . to_owned ()) } ; span_lint_and_sugg (cx , UNNECESSARY_FIRST_THEN_CHECK , sugg_span , format ! ("unnecessary use of `{both_calls_snippet}` to check if slice {}" , if is_some { "is not empty" } else { "is empty" }) , "replace this with" , suggestion , Applicability :: MachineApplicable ,) ; } }
};
}
