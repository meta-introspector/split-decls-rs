// Generated macro for merge_lines (function)
macro_rules! Depcrate_utils_doc_commentsmerge_lines {
() => {
// Module: crate::utils::doc_comments
// Provides: {"merge_lines"}
// Dependencies: {}
# [cfg (not (feature = "unstable-markdown"))] fn merge_lines (lines : impl IntoIterator < Item = impl AsRef < str > >) -> String { lines . into_iter () . map (| s | s . as_ref () . trim () . to_owned ()) . collect :: < Vec < _ > > () . join (" ") }
};
}
