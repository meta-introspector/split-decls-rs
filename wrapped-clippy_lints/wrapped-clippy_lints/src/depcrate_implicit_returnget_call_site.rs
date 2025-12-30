// Generated macro for get_call_site (function)
macro_rules! Depcrate_implicit_returnget_call_site {
() => {
// Module: crate::implicit_return
// Provides: {"get_call_site"}
// Dependencies: {}
fn get_call_site (span : Span , ctxt : SyntaxContext) -> Option < Span > { (span . ctxt () != ctxt) . then (| | walk_span_to_context (span , ctxt) . unwrap_or (span)) }
};
}
