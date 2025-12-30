// Generated macro for quote_span (function)
macro_rules! Depcrate_quotequote_span {
() => {
// Module: crate::quote
// Provides: {"quote_span"}
// Dependencies: {}
# [doc = " Quote a `Span` into a `TokenStream`."] # [doc = " This is needed to implement a custom quoter."] # [unstable (feature = "proc_macro_quote" , issue = "54722")] pub fn quote_span (proc_macro_crate : TokenStream , span : Span) -> TokenStream { let id = span . save_span () ; minimal_quote ! ((@ proc_macro_crate) :: Span :: recover_proc_macro_span ((@ TokenTree :: from (Literal :: usize_unsuffixed (id))))) }
};
}
