// Generated macro for check (function)
macro_rules! Depcrate_doc_link_with_quotescheck {
() => {
// Module: crate::doc::link_with_quotes
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , trimmed_text : & str , range : Range < usize > , fragments : Fragments < '_ >) { if ((trimmed_text . starts_with ('\'') && trimmed_text . ends_with ('\'')) || (trimmed_text . starts_with ('"') && trimmed_text . ends_with ('"'))) && let Some (span) = fragments . span (cx , range) { span_lint (cx , DOC_LINK_WITH_QUOTES , span , "possible intra-doc link using quotes instead of backticks" ,) ; } }
};
}
