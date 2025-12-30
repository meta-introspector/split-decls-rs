// Generated macro for reindent_snippet_if_in_block (function)
macro_rules! Depcrate_matches_match_single_bindingreindent_snippet_if_in_block {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"reindent_snippet_if_in_block"}
// Dependencies: {}
fn reindent_snippet_if_in_block (snippet_body : & str , has_assignment : bool) -> String { if has_assignment || ! snippet_body . starts_with ('{') { return reindent_multiline (snippet_body , true , indent_of_nth_line (snippet_body , 1)) ; } let snippet_body = snippet_body . trim_start_matches ('{') . trim_end_matches ('}') . trim () ; reindent_multiline (snippet_body , false , indent_of_nth_line (snippet_body , 0) . map (| indent | indent . saturating_sub (4)) ,) }
};
}
