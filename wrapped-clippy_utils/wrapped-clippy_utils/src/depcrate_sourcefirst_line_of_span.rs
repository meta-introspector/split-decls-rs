// Generated macro for first_line_of_span (function)
macro_rules! Depcrate_sourcefirst_line_of_span {
() => {
// Module: crate::source
// Provides: {"first_line_of_span"}
// Dependencies: {}
# [doc = " Returns a new Span that extends the original Span to the first non-whitespace char of the first"] # [doc = " line."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = "     let x = ();"] # [doc = " //          ^^"] # [doc = " // will be converted to"] # [doc = "     let x = ();"] # [doc = " //  ^^^^^^^^^^"] # [doc = " ```"] pub fn first_line_of_span (sess : & impl HasSession , span : Span) -> Span { first_char_in_first_line (sess , span) . map_or (span , | first_char_pos | span . with_lo (first_char_pos)) }
};
}
