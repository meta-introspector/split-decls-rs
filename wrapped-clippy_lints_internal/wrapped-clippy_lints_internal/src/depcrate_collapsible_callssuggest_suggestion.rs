// Generated macro for suggest_suggestion (function)
macro_rules! Depcrate_collapsible_callssuggest_suggestion {
() => {
// Module: crate::collapsible_calls
// Provides: {"suggest_suggestion"}
// Dependencies: {}
fn suggest_suggestion (cx : & LateContext < '_ > , expr : & Expr < '_ > , and_then_snippets : & AndThenSnippets < '_ > , span_suggestion_snippets : & SpanSuggestionSnippets < '_ > ,) { span_lint_and_sugg (cx , COLLAPSIBLE_SPAN_LINT_CALLS , expr . span , "this call is collapsible" , "collapse into" , format ! ("span_lint_and_sugg({}, {}, {}, {}, {}, {}, {})" , and_then_snippets . cx , and_then_snippets . lint , and_then_snippets . span , and_then_snippets . msg , span_suggestion_snippets . help , span_suggestion_snippets . sugg , span_suggestion_snippets . applicability) , Applicability :: MachineApplicable ,) ; }
};
}
