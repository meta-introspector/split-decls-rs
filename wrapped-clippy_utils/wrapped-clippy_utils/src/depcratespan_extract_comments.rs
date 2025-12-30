// Generated macro for span_extract_comments (function)
macro_rules! Depcratespan_extract_comments {
() => {
// Module: crate
// Provides: {"span_extract_comments"}
// Dependencies: {}
# [doc = " Returns all the comments a given span contains."] # [doc = ""] # [doc = " Comments are returned wrapped with their relevant delimiters."] pub fn span_extract_comments (sm : & SourceMap , span : Span) -> Vec < String > { let snippet = sm . span_to_snippet (span) . unwrap_or_default () ; tokenize_with_text (& snippet) . filter (| (t , ..) | matches ! (t , TokenKind :: BlockComment { .. } | TokenKind :: LineComment { .. })) . map (| (_ , s , _) | s . to_string ()) . collect :: < Vec < _ > > () }
};
}
