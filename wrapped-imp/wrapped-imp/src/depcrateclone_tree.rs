// Generated macro for clone_tree (function)
macro_rules! Depcrateclone_tree {
() => {
// Module: crate
// Provides: {"clone_tree"}
// Dependencies: {}
fn clone_tree (t : TokenTree) -> TokenTree { match t { TokenTree :: Group (orig) => { let mut new = Group :: new (orig . delimiter () , clone_stream (orig . stream ())) ; new . set_span (orig . span ()) ; TokenTree :: Group (new) } TokenTree :: Ident (orig) => { let s = orig . to_string () ; if let Some (rest) = s . strip_prefix ("r#") { TokenTree :: Ident (Ident :: new_raw (rest , orig . span ())) } else { TokenTree :: Ident (Ident :: new (& s , orig . span ())) } } TokenTree :: Punct (orig) => { let mut new = Punct :: new (orig . as_char () , orig . spacing ()) ; new . set_span (orig . span ()) ; TokenTree :: Punct (new) } TokenTree :: Literal (orig) => { let mut new : Literal = orig . to_string () . parse () . unwrap () ; new . set_span (orig . span ()) ; TokenTree :: Literal (new) } } }
};
}
