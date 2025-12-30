// Generated macro for fn_like_span_ops (function)
macro_rules! Depcratefn_like_span_ops {
() => {
// Module: crate
// Provides: {"fn_like_span_ops"}
// Dependencies: {}
# [proc_macro] pub fn fn_like_span_ops (args : TokenStream) -> TokenStream { let args = & mut args . into_iter () ; let mut first = args . next () . unwrap () ; first . set_span (Span :: def_site ()) ; let mut second = args . next () . unwrap () ; second . set_span (second . span () . resolved_at (Span :: def_site ())) ; let mut third = args . next () . unwrap () ; third . set_span (third . span () . start ()) ; TokenStream :: from_iter (vec ! [first , second , third]) }
};
}
