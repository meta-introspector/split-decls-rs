// Generated macro for snippet_block (function)
macro_rules! Depcrate_sourcesnippet_block {
() => {
// Module: crate::source
// Provides: {"snippet_block"}
// Dependencies: {}
# [doc = " Converts a span (from a block) to a code snippet if available, otherwise use default."] # [doc = ""] # [doc = " This trims the code of indentation, except for the first line. Use it for blocks or block-like"] # [doc = " things which need to be printed as such."] # [doc = ""] # [doc = " The `indent_relative_to` arg can be used, to provide a span, where the indentation of the"] # [doc = " resulting snippet of the given span."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " snippet_block(cx, block.span, \"..\", None)"] # [doc = " // where, `block` is the block of the if expr"] # [doc = "     if x {"] # [doc = "         y;"] # [doc = "     }"] # [doc = " // will return the snippet"] # [doc = " {"] # [doc = "     y;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " snippet_block(cx, block.span, \"..\", Some(if_expr.span))"] # [doc = " // where, `block` is the block of the if expr"] # [doc = "     if x {"] # [doc = "         y;"] # [doc = "     }"] # [doc = " // will return the snippet"] # [doc = " {"] # [doc = "         y;"] # [doc = "     } // aligned with `if`"] # [doc = " ```"] # [doc = " Note that the first line of the snippet always has 0 indentation."] pub fn snippet_block (sess : & impl HasSession , span : Span , default : & str , indent_relative_to : Option < Span >) -> String { let snip = snippet (sess , span , default) ; let indent = indent_relative_to . and_then (| s | indent_of (sess , s)) ; reindent_multiline (& snip , true , indent) }
};
}
