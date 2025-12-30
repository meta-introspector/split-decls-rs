// Generated macro for span_to_snippet_contains_docs (function)
macro_rules! Depcrate_missing_docspan_to_snippet_contains_docs {
() => {
// Module: crate::missing_doc
// Provides: {"span_to_snippet_contains_docs"}
// Dependencies: {}
fn span_to_snippet_contains_docs (cx : & LateContext < '_ > , search_span : Span) -> bool { search_span . check_source_text (cx , | src | src . lines () . rev () . any (| line | line . trim () . starts_with ("///"))) }
};
}
