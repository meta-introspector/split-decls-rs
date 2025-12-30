// Generated macro for impl_36 (impl)
macro_rules! Depcrate_ast_dataimpl_36 {
() => {
// Module: crate::ast::data
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Fields < T > { fn to_tokens (& self , tokens : & mut TokenStream) { let fields = & self . fields ; let span = self . span . unwrap_or_else (Span :: call_site) ; match self . style { Style :: Struct => { let trailing_comma = { if fields . is_empty () { quote ! () } else { quote ! (,) } } ; tokens . extend (quote_spanned ! [span => { # (# fields) ,* # trailing_comma }]) ; } Style :: Tuple => { tokens . extend (quote_spanned ! [span => (# (# fields) ,*)]) ; } Style :: Unit => { } } } }
};
}
