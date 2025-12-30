// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl ToTokens for Diagnostic { fn to_tokens (& self , dst : & mut TokenStream) { match & self . inner { Repr :: Single { text , span } => { let cs2 = (Span :: call_site () , Span :: call_site ()) ; let (start , end) = span . unwrap_or (cs2) ; dst . append (Ident :: new ("compile_error" , start)) ; dst . append (Punct :: new ('!' , Spacing :: Alone)) ; let mut message = TokenStream :: new () ; message . append (Literal :: string (text)) ; let mut group = Group :: new (Delimiter :: Brace , message) ; group . set_span (end) ; dst . append (group) ; } Repr :: Multi { diagnostics } => { for diagnostic in diagnostics { diagnostic . to_tokens (dst) ; } } Repr :: SynError (err) => { err . to_compile_error () . to_tokens (dst) ; } } } }
};
}
