macro_rules! deps {
    () => {
        AstChildren!();
        SyntaxNode!();
        SyntaxToken!();
        AstNode!();
        Language!();
    };
}

macro_rules! support {
    () => {
        deps!();
        pub mod support { use super :: { AstChildren , AstNode } ; use crate :: { Language , SyntaxNode , SyntaxToken } ; pub fn child < N : AstNode > (parent : & SyntaxNode < N :: Language >) -> Option < N > { parent . children () . find_map (N :: cast) } pub fn children < N : AstNode > (parent : & SyntaxNode < N :: Language >) -> AstChildren < N > { AstChildren :: new (parent) } pub fn token < L : Language > (parent : & SyntaxNode < L > , kind : L :: Kind) -> Option < SyntaxToken < L > > { parent . children_with_tokens () . filter_map (| it | it . into_token ()) . find (| it | it . kind () == kind) } }
    };
}

support!();