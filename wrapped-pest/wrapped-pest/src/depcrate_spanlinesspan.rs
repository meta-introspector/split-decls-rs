// Generated macro for LinesSpan (struct)
macro_rules! Depcrate_spanLinesSpan {
() => {
// Module: crate::span
// Provides: {"LinesSpan"}
// Dependencies: {}
# [doc = " Line iterator for Spans, created by [`Span::lines_span()`]."] # [doc = ""] # [doc = " Iterates all lines that are at least _partially_ covered by the span. Yielding a `Span` for each."] # [doc = ""] # [doc = " [`Span::lines_span()`]: struct.Span.html#method.lines_span"] pub struct LinesSpan < 'i > { span : & 'i Span < 'i > , pos : usize , }
};
}
