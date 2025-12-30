// Generated macro for snippet_block_with_context (function)
macro_rules! Depcrate_sourcesnippet_block_with_context {
() => {
// Module: crate::source
// Provides: {"snippet_block_with_context"}
// Dependencies: {}
pub fn snippet_block_with_context (sess : & impl HasSession , span : Span , outer : SyntaxContext , default : & str , indent_relative_to : Option < Span > , app : & mut Applicability ,) -> (String , bool) { let (snip , from_macro) = snippet_with_context (sess , span , outer , default , app) ; let indent = indent_relative_to . and_then (| s | indent_of (sess , s)) ; (reindent_multiline (& snip , true , indent) , from_macro) }
};
}
