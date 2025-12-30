// Generated macro for impl_27 (impl)
macro_rules! Depcrate_diagnosticimpl_27 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_27"}
// Dependencies: {}
impl ToTokens for Diagnostic { fn to_tokens (& self , ts : & mut TokenStream) { use std :: borrow :: Cow ; fn ensure_lf (buf : & mut String , s : & str) { if s . ends_with ('\n') { buf . push_str (s) ; } else { buf . push_str (s) ; buf . push ('\n') ; } } fn diag_to_tokens (span_range : SpanRange , level : & Level , msg : & str , suggestions : & [(SuggestionKind , String , Option < SpanRange >)] ,) -> TokenStream { if * level == Level :: Warning { return TokenStream :: new () ; } let message = if suggestions . is_empty () { Cow :: Borrowed (msg) } else { let mut message = String :: new () ; ensure_lf (& mut message , msg) ; message . push ('\n') ; for (kind , note , _span) in suggestions { message . push_str ("  = ") ; message . push_str (kind . name ()) ; message . push_str (": ") ; ensure_lf (& mut message , note) ; } message . push ('\n') ; Cow :: Owned (message) } ; let mut msg = proc_macro2 :: Literal :: string (& message) ; msg . set_span (span_range . last) ; let group = quote_spanned ! (span_range . last => { # msg }) ; quote_spanned ! (span_range . first => compile_error !# group) } ts . extend (diag_to_tokens (self . span_range , & self . level , & self . msg , & self . suggestions ,)) ; ts . extend (self . children . iter () . map (| (span_range , msg) | diag_to_tokens (* span_range , & Level :: Error , msg , & [])) ,) ; } }
};
}
