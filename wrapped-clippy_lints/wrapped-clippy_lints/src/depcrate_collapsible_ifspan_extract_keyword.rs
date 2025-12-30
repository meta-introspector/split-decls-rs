// Generated macro for span_extract_keyword (function)
macro_rules! Depcrate_collapsible_ifspan_extract_keyword {
() => {
// Module: crate::collapsible_if
// Provides: {"span_extract_keyword"}
// Dependencies: {}
fn span_extract_keyword (sm : & SourceMap , span : Span , keyword : & str) -> Option < Span > { let snippet = sm . span_to_snippet (span) . ok () ? ; tokenize_with_text (& snippet) . filter (| (t , s , _) | matches ! (t , TokenKind :: Ident if * s == keyword)) . map (| (_ , _ , inner) | { span . split_at (u32 :: try_from (inner . start) . unwrap ()) . 1 . split_at (u32 :: try_from (inner . end - inner . start) . unwrap ()) . 0 }) . next () }
};
}
