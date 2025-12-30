// Generated macro for filter_serde_attrs (function)
macro_rules! Depcratefilter_serde_attrs {
() => {
// Module: crate
// Provides: {"filter_serde_attrs"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn filter_serde_attrs (attrs : & [Attribute]) -> bool { fn contains_skip (tokens : TokenStream2) -> bool { for token in tokens . into_iter () { match token { TokenTree :: Group (group) => { if contains_skip (group . stream ()) { return true ; } } TokenTree :: Ident (ident) => { if ident == "skip" { return true ; } } TokenTree :: Punct (_) | TokenTree :: Literal (_) => () , } } false } for attr in attrs { if ! attr . path () . is_ident ("serde") { continue ; } if contains_skip (attr . to_token_stream ()) { return true ; } } false }
};
}
