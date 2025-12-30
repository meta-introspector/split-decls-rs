// Generated macro for fn_like_mk_idents (function)
macro_rules! Depcratefn_like_mk_idents {
() => {
// Module: crate
// Provides: {"fn_like_mk_idents"}
// Dependencies: {}
# [proc_macro] pub fn fn_like_mk_idents (_args : TokenStream) -> TokenStream { let trees : Vec < TokenTree > = vec ! [TokenTree :: from (Ident :: new ("standard" , Span :: call_site ())) , TokenTree :: from (Ident :: new_raw ("raw" , Span :: call_site ())) ,] ; TokenStream :: from_iter (trees) }
};
}
