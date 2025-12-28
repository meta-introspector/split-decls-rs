macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! only_ident {
    () => {
        deps!();
        fn only_ident (element : SyntaxElement) -> Option < SyntaxToken > { match element { SyntaxElement :: Token (t) => { if t . kind () == SyntaxKind :: IDENT { return Some (t) ; } } SyntaxElement :: Node (n) => { let mut children = n . children_with_tokens () ; if let (Some (only_child) , None) = (children . next () , children . next ()) { return only_ident (only_child) ; } } } None }
    };
}

only_ident!();