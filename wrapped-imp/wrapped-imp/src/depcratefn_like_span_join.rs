// Generated macro for fn_like_span_join (function)
macro_rules! Depcratefn_like_span_join {
() => {
// Module: crate
// Provides: {"fn_like_span_join"}
// Dependencies: {}
# [proc_macro] pub fn fn_like_span_join (args : TokenStream) -> TokenStream { let args = & mut args . into_iter () ; let first = args . next () . unwrap () ; let second = args . next () . unwrap () ; TokenStream :: from (TokenTree :: from (Ident :: new_raw ("joined" , first . span () . join (second . span ()) . unwrap () ,))) }
};
}
