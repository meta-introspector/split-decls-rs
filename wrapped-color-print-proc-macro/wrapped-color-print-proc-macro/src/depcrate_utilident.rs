// Generated macro for ident (function)
macro_rules! Depcrate_utilident {
() => {
// Module: crate::util
// Provides: {"ident"}
// Dependencies: {}
# [doc = " Creates a new [`Ident`] which can be tokenized."] pub fn ident (s : & str) -> Ident { Ident :: new (s , Span :: call_site ()) }
};
}
