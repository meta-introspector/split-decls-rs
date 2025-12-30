// Generated macro for impl_267 (impl)
macro_rules! Depcrateimpl_267 {
() => {
// Module: crate
// Provides: {"impl_267"}
// Dependencies: {}
impl TokenTree { # [doc = " Returns the span of this tree, delegating to the `span` method of"] # [doc = " the contained token or a delimited stream."] pub fn span (& self) -> Span { match self { TokenTree :: Group (t) => t . span () , TokenTree :: Ident (t) => t . span () , TokenTree :: Punct (t) => t . span () , TokenTree :: Literal (t) => t . span () , } } # [doc = " Configures the span for *only this token*."] # [doc = ""] # [doc = " Note that if this token is a `Group` then this method will not configure"] # [doc = " the span of each of the internal tokens, this will simply delegate to"] # [doc = " the `set_span` method of each variant."] pub fn set_span (& mut self , span : Span) { match self { TokenTree :: Group (t) => t . set_span (span) , TokenTree :: Ident (t) => t . set_span (span) , TokenTree :: Punct (t) => t . set_span (span) , TokenTree :: Literal (t) => t . set_span (span) , } } }
};
}
