// Generated macro for get_idents (function)
macro_rules! Depcrateget_idents {
() => {
// Module: crate
// Provides: {"get_idents"}
// Dependencies: {}
# [doc = " Extract all identifiers in the given tokenstream."] fn get_idents (input : TokenStream) -> impl Iterator < Item = Ident > { input . into_iter () . flat_map (| token | { match token { TokenTree :: Group (group) => get_idents (group . stream ()) . collect :: < Vec < _ > > () , TokenTree :: Ident (ident) => { vec ! [ident] } TokenTree :: Punct (_) | TokenTree :: Literal (_) => { vec ! [] } } . into_iter () }) }
};
}
