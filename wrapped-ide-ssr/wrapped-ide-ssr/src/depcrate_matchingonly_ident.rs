// Generated macro for only_ident (function)
macro_rules! Depcrate_matchingonly_ident {
() => {
// Module: crate::matching
// Provides: {"only_ident"}
// Dependencies: {}
fn only_ident (element : SyntaxElement) -> Option < SyntaxToken > { match element { SyntaxElement :: Token (t) => { if t . kind () == SyntaxKind :: IDENT { return Some (t) ; } } SyntaxElement :: Node (n) => { let mut children = n . children_with_tokens () ; if let (Some (only_child) , None) = (children . next () , children . next ()) { return only_ident (only_child) ; } } } None }
};
}
