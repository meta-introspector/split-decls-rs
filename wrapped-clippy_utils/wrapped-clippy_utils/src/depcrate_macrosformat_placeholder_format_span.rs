// Generated macro for format_placeholder_format_span (function)
macro_rules! Depcrate_macrosformat_placeholder_format_span {
() => {
// Module: crate::macros
// Provides: {"format_placeholder_format_span"}
// Dependencies: {}
# [doc = " Span of the `:` and format specifiers"] # [doc = ""] # [doc = " ```ignore"] # [doc = " format!(\"{:.}\"), format!(\"{foo:.}\")"] # [doc = "           ^^                  ^^"] # [doc = " ```"] pub fn format_placeholder_format_span (placeholder : & FormatPlaceholder) -> Option < Span > { let base = placeholder . span ? . data () ; Some (Span :: new (placeholder . argument . span ? . hi () , base . hi - BytePos (1) , base . ctxt , base . parent ,)) }
};
}
