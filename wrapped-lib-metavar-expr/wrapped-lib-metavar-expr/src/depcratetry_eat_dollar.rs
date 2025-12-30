// Generated macro for try_eat_dollar (function)
macro_rules! Depcratetry_eat_dollar {
() => {
// Module: crate
// Provides: {"try_eat_dollar"}
// Dependencies: {}
# [doc = " Tries to move the iterator forward returning `true` if there is a dollar sign. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_dollar (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Dollar , .. } , _)) = iter . peek () { let _ = iter . next () ; return true ; } false }
};
}
