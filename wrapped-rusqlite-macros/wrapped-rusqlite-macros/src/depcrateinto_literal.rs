// Generated macro for into_literal (function)
macro_rules! Depcrateinto_literal {
() => {
// Module: crate
// Provides: {"into_literal"}
// Dependencies: {}
fn into_literal (ts : & TokenTree) -> Option < Literal > { match ts { TokenTree :: Literal (l) => Some (l . clone ()) , TokenTree :: Group (g) => match g . delimiter () { Delimiter :: None => match g . stream () . into_iter () . collect :: < Vec < _ > > () . as_slice () { [TokenTree :: Literal (l)] => Some (l . clone ()) , _ => None , } , Delimiter :: Parenthesis | Delimiter :: Brace | Delimiter :: Bracket => None , } , _ => None , } }
};
}
