// Generated macro for lint_return (function)
macro_rules! Depcrate_implicit_returnlint_return {
() => {
// Module: crate::implicit_return
// Provides: {"lint_return"}
// Dependencies: {}
fn lint_return (cx : & LateContext < '_ > , emission_place : HirId , span : Span) { span_lint_hir_and_then (cx , IMPLICIT_RETURN , emission_place , span , "missing `return` statement" , | diag | { let mut app = Applicability :: MachineApplicable ; let snip = snippet_with_applicability (cx , span , ".." , & mut app) ; diag . span_suggestion_verbose (span , "add `return` as shown" , format ! ("return {snip}") , app) ; } ,) ; }
};
}
