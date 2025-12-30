// Generated macro for suggest_help (function)
macro_rules! Depcrate_collapsible_callssuggest_help {
() => {
// Module: crate::collapsible_calls
// Provides: {"suggest_help"}
// Dependencies: {}
fn suggest_help (cx : & LateContext < '_ > , expr : & Expr < '_ > , and_then_snippets : & AndThenSnippets < '_ > , help : & str , with_span : bool ,) { let option_span = if with_span { format ! ("Some({})" , and_then_snippets . span) } else { "None" . to_string () } ; span_lint_and_sugg (cx , COLLAPSIBLE_SPAN_LINT_CALLS , expr . span , "this call is collapsible" , "collapse into" , format ! ("span_lint_and_help({}, {}, {}, {}, {}, {help})" , and_then_snippets . cx , and_then_snippets . lint , and_then_snippets . span , and_then_snippets . msg , & option_span ,) , Applicability :: MachineApplicable ,) ; }
};
}
