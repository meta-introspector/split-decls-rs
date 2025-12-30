// Generated macro for get_and_then_snippets (function)
macro_rules! Depcrate_collapsible_callsget_and_then_snippets {
() => {
// Module: crate::collapsible_calls
// Provides: {"get_and_then_snippets"}
// Dependencies: {}
fn get_and_then_snippets (cx : & LateContext < '_ > , cx_span : Span , lint_span : Span , span_span : Span , msg_span : Span ,) -> AndThenSnippets < 'static > { let cx_snippet = snippet (cx , cx_span , "cx") ; let lint_snippet = snippet (cx , lint_span , "..") ; let span_snippet = snippet (cx , span_span , "span") ; let msg_snippet = snippet (cx , msg_span , r#""...""#) ; AndThenSnippets { cx : cx_snippet , lint : lint_snippet , span : span_snippet , msg : msg_snippet , } }
};
}
