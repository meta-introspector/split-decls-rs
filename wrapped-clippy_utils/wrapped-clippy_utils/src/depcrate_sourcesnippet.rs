// Generated macro for snippet (function)
macro_rules! Depcrate_sourcesnippet {
() => {
// Module: crate::source
// Provides: {"snippet"}
// Dependencies: {}
# [doc = " Converts a span to a code snippet if available, otherwise returns the default."] # [doc = ""] # [doc = " This is useful if you want to provide suggestions for your lint or more generally, if you want"] # [doc = " to convert a given `Span` to a `str`. To create suggestions consider using"] # [doc = " [`snippet_with_applicability`] to ensure that the applicability stays correct."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " // Given two spans one for `value` and one for the `init` expression."] # [doc = " let value = Vec::new();"] # [doc = " //  ^^^^^   ^^^^^^^^^^"] # [doc = " //  span1   span2"] # [doc = ""] # [doc = " // The snipped call would return the corresponding code snippet"] # [doc = " snippet(cx, span1, \"..\") // -> \"value\""] # [doc = " snippet(cx, span2, \"..\") // -> \"Vec::new()\""] # [doc = " ```"] pub fn snippet < 'a > (sess : & impl HasSession , span : Span , default : & 'a str) -> Cow < 'a , str > { snippet_opt (sess , span) . map_or_else (| | Cow :: Borrowed (default) , From :: from) }
};
}
