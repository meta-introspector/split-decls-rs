// Generated macro for snippet_block_with_applicability (function)
macro_rules! Depcrate_sourcesnippet_block_with_applicability {
() => {
// Module: crate::source
// Provides: {"snippet_block_with_applicability"}
// Dependencies: {}
# [doc = " Same as `snippet_block`, but adapts the applicability level by the rules of"] # [doc = " `snippet_with_applicability`."] pub fn snippet_block_with_applicability (sess : & impl HasSession , span : Span , default : & str , indent_relative_to : Option < Span > , applicability : & mut Applicability ,) -> String { let snip = snippet_with_applicability (sess , span , default , applicability) ; let indent = indent_relative_to . and_then (| s | indent_of (sess , s)) ; reindent_multiline (& snip , true , indent) }
};
}
