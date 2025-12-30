// Generated macro for collect_doc_replacements (function)
macro_rules! Depcrate_doc_suspicious_doc_commentscollect_doc_replacements {
() => {
// Module: crate::doc::suspicious_doc_comments
// Provides: {"collect_doc_replacements"}
// Dependencies: {}
fn collect_doc_replacements (attrs : & [Attribute]) -> Vec < (Span , String) > { attrs . iter () . filter_map (| attr | { if let Attribute :: Parsed (AttributeKind :: DocComment { style : AttrStyle :: Outer , kind , comment , .. }) = attr && let Some (com) = comment . as_str () . strip_prefix ('!') { let sugg = match kind { CommentKind :: Line => format ! ("//!{com}") , CommentKind :: Block => format ! ("/*!{com}*/") , } ; Some ((attr . span () , sugg)) } else { None } }) . collect () }
};
}
