// Generated macro for ident (function)
macro_rules! Depcrateident {
() => {
// Module: crate
// Provides: {"ident"}
// Dependencies: {}
# [doc = " Create a TokenStream of an identifier out of a string"] fn ident (ident : & str) -> TokenStream { TokenTree :: from (Ident :: new (ident , Span :: call_site ())) . into () }
};
}
