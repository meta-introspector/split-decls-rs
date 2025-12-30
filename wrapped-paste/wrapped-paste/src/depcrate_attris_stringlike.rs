// Generated macro for is_stringlike (function)
macro_rules! Depcrate_attris_stringlike {
() => {
// Module: crate::attr
// Provides: {"is_stringlike"}
// Dependencies: {}
fn is_stringlike (token : & TokenTree) -> bool { match token { TokenTree :: Ident (_) => true , TokenTree :: Literal (literal) => { let repr = literal . to_string () ; ! repr . starts_with ('b') && ! repr . starts_with ('\'') } TokenTree :: Group (group) => { if group . delimiter () != Delimiter :: None { return false ; } let mut inner = group . stream () . into_iter () ; match inner . next () { Some (first) => inner . next () . is_none () && is_stringlike (& first) , None => false , } } TokenTree :: Punct (punct) => { punct . as_char () == '\'' || punct . as_char () == ':' && punct . spacing () == Spacing :: Alone } } }
};
}
