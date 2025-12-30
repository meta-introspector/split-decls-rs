// Generated macro for is_crate_keyword (function)
macro_rules! Depcrate_crate_in_macro_defis_crate_keyword {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"is_crate_keyword"}
// Dependencies: {}
fn is_crate_keyword (tt : & TokenTree) -> Option < Span > { if let TokenTree :: Token (Token { kind : TokenKind :: Ident (kw :: Crate , _) , span , } , _ ,) = tt { Some (* span) } else { None } }
};
}
