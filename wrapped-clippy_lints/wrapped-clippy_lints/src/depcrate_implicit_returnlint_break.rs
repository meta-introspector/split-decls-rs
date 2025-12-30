// Generated macro for lint_break (function)
macro_rules! Depcrate_implicit_returnlint_break {
() => {
// Module: crate::implicit_return
// Provides: {"lint_break"}
// Dependencies: {}
fn lint_break (cx : & LateContext < '_ > , emission_place : HirId , break_span : Span , expr_span : Span) { span_lint_hir_and_then (cx , IMPLICIT_RETURN , emission_place , break_span , "missing `return` statement" , | diag | { let mut app = Applicability :: MachineApplicable ; let snip = snippet_with_context (cx , expr_span , break_span . ctxt () , ".." , & mut app) . 0 ; diag . span_suggestion_verbose (break_span , "change `break` to `return` as shown" , format ! ("return {snip}") , app ,) ; } ,) ; }
};
}
