// Generated macro for check (function)
macro_rules! Depcrate_doc_doc_comment_double_space_linebreakscheck {
() => {
// Module: crate::doc::doc_comment_double_space_linebreaks
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , collected_breaks : & [Span]) { if collected_breaks . is_empty () { return ; } let breaks : Vec < _ > = collected_breaks . iter () . map (| span | span . with_hi (span . lo () + BytePos (2))) . collect () ; span_lint_and_then (cx , DOC_COMMENT_DOUBLE_SPACE_LINEBREAKS , breaks . clone () , "doc comment uses two spaces for a hard line break" , | diag | { let suggs : Vec < _ > = breaks . iter () . map (| span | (* span , "\\" . to_string ())) . collect () ; diag . tool_only_multipart_suggestion ("replace this double space with a backslash:" , suggs , Applicability :: MachineApplicable ,) ; diag . help ("replace this double space with a backslash: `\\`") ; } ,) ; }
};
}
