// Generated macro for get_may_borrow_attr (function)
macro_rules! Depcrateget_may_borrow_attr {
() => {
// Module: crate
// Provides: {"get_may_borrow_attr"}
// Dependencies: {}
fn get_may_borrow_attr (attrs : & [syn :: Attribute]) -> Result < HashSet < Ident > , Span > { let mut params = HashSet :: new () ; for attr in attrs { if let Ok (list) = attr . parse_args :: < MetaList > () { if list . path . is_ident ("may_borrow") { if let Ok (list) = list . parse_args_with (Punctuated :: < Ident , Token ! [,] > :: parse_terminated) { params . extend (list) } else { return Err (attr . span ()) ; } } } } Ok (params) }
};
}
