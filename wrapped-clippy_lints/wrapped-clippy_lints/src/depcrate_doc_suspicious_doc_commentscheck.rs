// Generated macro for check (function)
macro_rules! Depcrate_doc_suspicious_doc_commentscheck {
() => {
// Module: crate::doc::suspicious_doc_comments
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , attrs : & [Attribute]) -> bool { let replacements : Vec < _ > = collect_doc_replacements (attrs) ; if let Some ((& (lo_span , _) , & (hi_span , _))) = replacements . first () . zip (replacements . last ()) { span_lint_and_then (cx , SUSPICIOUS_DOC_COMMENTS , lo_span . to (hi_span) , "this is an outer doc comment and does not apply to the parent module or crate" , | diag | { diag . multipart_suggestion ("use an inner doc comment to document the parent module or crate" , replacements , Applicability :: MaybeIncorrect ,) ; } ,) ; true } else { false } }
};
}
