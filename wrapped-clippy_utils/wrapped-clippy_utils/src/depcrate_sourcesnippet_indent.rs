// Generated macro for snippet_indent (function)
macro_rules! Depcrate_sourcesnippet_indent {
() => {
// Module: crate::source
// Provides: {"snippet_indent"}
// Dependencies: {}
# [doc = " Gets a snippet of the indentation of the line of a span"] pub fn snippet_indent (sess : & impl HasSession , span : Span) -> Option < String > { snippet_opt (sess , line_span (sess , span)) . map (| mut s | { let len = s . len () - s . trim_start () . len () ; s . truncate (len) ; s }) }
};
}
