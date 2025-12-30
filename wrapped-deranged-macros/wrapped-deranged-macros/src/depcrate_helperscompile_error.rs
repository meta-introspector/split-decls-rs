// Generated macro for compile_error (function)
macro_rules! Depcrate_helperscompile_error {
() => {
// Module: crate::helpers
// Provides: {"compile_error"}
// Dependencies: {}
# [doc = " Construct a compilation error with the provided message."] pub (crate) fn compile_error (message : & str , span : impl MaybeSpan) -> TokenStream { let (span_start , span_end) = span . into_pair () ; TokenStream :: from_iter ([with_span (TokenTree :: from (Punct :: new (':' , Spacing :: Joint)) , span_start) , with_span (TokenTree :: from (Punct :: new (':' , Spacing :: Alone)) , span_start) , TokenTree :: from (Ident :: new ("core" , span_start)) , with_span (TokenTree :: from (Punct :: new (':' , Spacing :: Joint)) , span_start) , with_span (TokenTree :: from (Punct :: new (':' , Spacing :: Alone)) , span_start) , with_span (TokenTree :: from (Ident :: new ("compile_error" , Span :: mixed_site ())) , span_start ,) , with_span (TokenTree :: from (Punct :: new ('!' , Spacing :: Alone)) , span_start) , with_span (TokenTree :: from (Group :: new (Delimiter :: Parenthesis , TokenStream :: from (TokenTree :: Literal (Literal :: string (message))) ,)) , span_end ,) ,]) }
};
}
