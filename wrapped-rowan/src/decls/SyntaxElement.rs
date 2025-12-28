macro_rules! deps {
    () => {
        NodeOrToken!();
        SyntaxToken!();
        SyntaxNode!();
    };
}

macro_rules! SyntaxElement {
    () => {
        deps!();
        pub type SyntaxElement < L > = NodeOrToken < SyntaxNode < L > , SyntaxToken < L > > ;
    };
}

SyntaxElement!()