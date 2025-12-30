// Generated macro for tree_to_bridge_tree (function)
macro_rules! Depcratetree_to_bridge_tree {
() => {
// Module: crate
// Provides: {"tree_to_bridge_tree"}
// Dependencies: {}
fn tree_to_bridge_tree (tree : TokenTree ,) -> bridge :: TokenTree < bridge :: client :: TokenStream , bridge :: client :: Span , bridge :: client :: Symbol > { match tree { TokenTree :: Group (tt) => bridge :: TokenTree :: Group (tt . 0) , TokenTree :: Punct (tt) => bridge :: TokenTree :: Punct (tt . 0) , TokenTree :: Ident (tt) => bridge :: TokenTree :: Ident (tt . 0) , TokenTree :: Literal (tt) => bridge :: TokenTree :: Literal (tt . 0) , } }
};
}
