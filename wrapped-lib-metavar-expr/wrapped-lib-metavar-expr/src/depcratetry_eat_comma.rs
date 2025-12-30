// Generated macro for try_eat_comma (function)
macro_rules! Depcratetry_eat_comma {
() => {
// Module: crate
// Provides: {"try_eat_comma"}
// Dependencies: {}
# [doc = " Tries to move the iterator forward returning `true` if there is a comma. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_comma (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Comma , .. } , _)) = iter . peek () { let _ = iter . next () ; return true ; } false }
};
}
