// Generated macro for check (function)
macro_rules! Depcrate_doc_doc_paragraphs_missing_punctuationcheck {
() => {
// Module: crate::doc::doc_paragraphs_missing_punctuation
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , doc : & str , fragments : Fragments < '_ >) { for missing_punctuation in is_missing_punctuation (doc) { match missing_punctuation { MissingPunctuation :: Fixable (offset) => { if let Some (span) = fragments . span (cx , offset .. offset) { clippy_utils :: diagnostics :: span_lint_and_sugg (cx , DOC_PARAGRAPHS_MISSING_PUNCTUATION , span , MSG , "end the paragraph with some punctuation" , PUNCTUATION_SUGGESTION . to_string () , Applicability :: MaybeIncorrect ,) ; } } , MissingPunctuation :: Unfixable (offset) => { if let Some (span) = fragments . span (cx , offset .. offset) { clippy_utils :: diagnostics :: span_lint_and_help (cx , DOC_PARAGRAPHS_MISSING_PUNCTUATION , span , MSG , None , "end the paragraph with some punctuation" ,) ; } } , } } }
};
}
