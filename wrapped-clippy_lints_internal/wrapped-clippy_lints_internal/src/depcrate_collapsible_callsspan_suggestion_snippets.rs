// Generated macro for span_suggestion_snippets (function)
macro_rules! Depcrate_collapsible_callsspan_suggestion_snippets {
() => {
// Module: crate::collapsible_calls
// Provides: {"span_suggestion_snippets"}
// Dependencies: {}
fn span_suggestion_snippets < 'a , 'hir > (cx : & LateContext < '_ > , span_call_args : & 'hir [Expr < 'hir >] ,) -> SpanSuggestionSnippets < 'a > { let help_snippet = snippet (cx , span_call_args [1] . span , r#""...""#) ; let sugg_snippet = snippet (cx , span_call_args [2] . span , "..") ; let applicability_snippet = snippet (cx , span_call_args [3] . span , "Applicability::MachineApplicable") ; SpanSuggestionSnippets { help : help_snippet , sugg : sugg_snippet , applicability : applicability_snippet , } }
};
}
