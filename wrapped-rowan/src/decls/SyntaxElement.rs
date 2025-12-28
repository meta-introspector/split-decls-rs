macro_rules! deps {
    () => {
        NodeOrToken!();
        SyntaxNode!();
        SyntaxToken!();
    };
}

macro_rules! SyntaxElement {
    () => {
        deps!();
        pub type SyntaxElement < L > = NodeOrToken < SyntaxNode < L > , SyntaxToken < L > > ;
    };
}

SyntaxElement!();