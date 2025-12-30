// Generated macro for impl_360 (impl)
macro_rules! Depcrateimpl_360 {
() => {
// Module: crate
// Provides: {"impl_360"}
// Dependencies: {}
impl TokenTree { # [doc = " Returns the span of this tree, delegating to the `span` method of"] # [doc = " the contained token or a delimited stream."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn span (& self) -> Span { match * self { TokenTree :: Group (ref t) => t . span () , TokenTree :: Ident (ref t) => t . span () , TokenTree :: Punct (ref t) => t . span () , TokenTree :: Literal (ref t) => t . span () , } } # [doc = " Configures the span for *only this token*."] # [doc = ""] # [doc = " Note that if this token is a `Group` then this method will not configure"] # [doc = " the span of each of the internal tokens, this will simply delegate to"] # [doc = " the `set_span` method of each variant."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn set_span (& mut self , span : Span) { match * self { TokenTree :: Group (ref mut t) => t . set_span (span) , TokenTree :: Ident (ref mut t) => t . set_span (span) , TokenTree :: Punct (ref mut t) => t . set_span (span) , TokenTree :: Literal (ref mut t) => t . set_span (span) , } } }
};
}
