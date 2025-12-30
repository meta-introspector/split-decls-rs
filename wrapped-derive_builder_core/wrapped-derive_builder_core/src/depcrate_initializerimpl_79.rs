// Generated macro for impl_79 (impl)
macro_rules! Depcrate_initializerimpl_79 {
() => {
// Module: crate::initializer
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > ToTokens for MatchNone < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { match * self { MatchNone :: DefaultTo { expr , crate_root } => { let expr = expr . with_crate_root (crate_root) ; tokens . append_all (quote ! (None => # expr)) ; } MatchNone :: UseDefaultStructField (field_ident) => { let struct_ident = syn :: Ident :: new (DEFAULT_STRUCT_NAME , Span :: call_site ()) ; tokens . append_all (quote ! (None => # struct_ident .# field_ident)) } MatchNone :: ReturnError { ref field_name , ref span , crate_root , } => { let conv_span = span . unwrap_or_else (Span :: call_site) ; let crate_root = change_span (crate_root . into_token_stream () , conv_span) ; let err_conv = quote_spanned ! (conv_span => # crate_root :: export :: core :: convert :: Into :: into (# crate_root :: UninitializedFieldError :: from (# field_name))) ; tokens . append_all (quote ! (None => return # crate_root :: export :: core :: result :: Result :: Err (# err_conv))) ; } } } }
};
}
