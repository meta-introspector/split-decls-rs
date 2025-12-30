// Generated macro for literal_string (function)
macro_rules! Depcrate_utilliteral_string {
() => {
// Module: crate::util
// Provides: {"literal_string"}
// Dependencies: {}
# [doc = " Creates a new [`struct@LitStr`] which can be tokenized."] pub fn literal_string (s : & str) -> LitStr { LitStr :: new (s , Span :: call_site ()) }
};
}
