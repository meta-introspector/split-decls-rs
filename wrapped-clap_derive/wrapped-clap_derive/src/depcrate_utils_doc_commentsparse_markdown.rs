// Generated macro for parse_markdown (function)
macro_rules! Depcrate_utils_doc_commentsparse_markdown {
() => {
// Module: crate::utils::doc_comments
// Provides: {"parse_markdown"}
// Dependencies: {}
# [cfg (not (feature = "unstable-markdown"))] fn parse_markdown (lines : & [String]) -> (String , Option < String >) { if lines . iter () . any (| s | is_blank (s)) { let paragraphs = split_paragraphs (lines) ; let short = paragraphs [0] . clone () ; let long = paragraphs . join ("\n\n") ; (short , Some (long)) } else { let short = merge_lines (lines) ; (short , None) } }
};
}
