// Generated macro for span_extract_comment (function)
macro_rules! Depcratespan_extract_comment {
() => {
// Module: crate
// Provides: {"span_extract_comment"}
// Dependencies: {}
# [doc = " Returns all the comments a given span contains"] # [doc = ""] # [doc = " Comments are returned wrapped with their relevant delimiters"] pub fn span_extract_comment (sm : & SourceMap , span : Span) -> String { span_extract_comments (sm , span) . join ("\n") }
};
}
