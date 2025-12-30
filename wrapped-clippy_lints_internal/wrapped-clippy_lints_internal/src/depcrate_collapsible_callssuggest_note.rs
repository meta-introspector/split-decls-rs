// Generated macro for suggest_note (function)
macro_rules! Depcrate_collapsible_callssuggest_note {
() => {
// Module: crate::collapsible_calls
// Provides: {"suggest_note"}
// Dependencies: {}
fn suggest_note (cx : & LateContext < '_ > , expr : & Expr < '_ > , and_then_snippets : & AndThenSnippets < '_ > , note : & str , with_span : bool ,) { let note_span = if with_span { format ! ("Some({})" , and_then_snippets . span) } else { "None" . to_string () } ; span_lint_and_sugg (cx , COLLAPSIBLE_SPAN_LINT_CALLS , expr . span , "this call is collapsible" , "collapse into" , format ! ("span_lint_and_note({}, {}, {}, {}, {note_span}, {note})" , and_then_snippets . cx , and_then_snippets . lint , and_then_snippets . span , and_then_snippets . msg ,) , Applicability :: MachineApplicable ,) ; }
};
}
