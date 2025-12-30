// Generated macro for snippet_opt (function)
macro_rules! Depcrate_sourcesnippet_opt {
() => {
// Module: crate::source
// Provides: {"snippet_opt"}
// Dependencies: {}
# [doc = " Converts a span to a code snippet. Returns `None` if not available."] pub fn snippet_opt (sess : & impl HasSession , span : Span) -> Option < String > { sess . sess () . source_map () . span_to_snippet (span) . ok () }
};
}
