// Generated macro for has_attr (function)
macro_rules! Depcratehas_attr {
() => {
// Module: crate
// Provides: {"has_attr"}
// Dependencies: {}
fn has_attr (attrs : & [syn :: Attribute] , name : & str) -> bool { attrs . iter () . any (| a | { if let Ok (i) = a . parse_args :: < Ident > () { if i == name { return true ; } } false }) }
};
}
