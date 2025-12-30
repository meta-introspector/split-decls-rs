// Generated macro for indent_of (function)
macro_rules! Depcrate_sourceindent_of {
() => {
// Module: crate::source
// Provides: {"indent_of"}
// Dependencies: {}
# [doc = " Returns the indentation of the line of a span"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " let x = ();"] # [doc = " //      ^^ -- will return 0"] # [doc = "     let x = ();"] # [doc = " //          ^^ -- will return 4"] # [doc = " ```"] pub fn indent_of (sess : & impl HasSession , span : Span) -> Option < usize > { snippet_opt (sess , line_span (sess , span)) . and_then (| snip | snip . find (| c : char | ! c . is_whitespace ())) }
};
}
