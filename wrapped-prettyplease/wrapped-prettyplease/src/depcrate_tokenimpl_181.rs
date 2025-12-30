// Generated macro for impl_181 (impl)
macro_rules! Depcrate_tokenimpl_181 {
() => {
// Module: crate::token
// Provides: {"impl_181"}
// Dependencies: {}
impl From < TokenTree > for Token { fn from (tt : TokenTree) -> Self { match tt { TokenTree :: Group (group) => Token :: Group (group . delimiter () , group . stream ()) , TokenTree :: Ident (ident) => Token :: Ident (ident) , TokenTree :: Punct (punct) => Token :: Punct (punct . as_char () , punct . spacing ()) , TokenTree :: Literal (literal) => Token :: Literal (literal) , } } }
};
}
