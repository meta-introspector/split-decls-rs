macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxNode!();
        NodeOrToken!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl From < SyntaxNode > for SyntaxElement { # [inline] fn from (node : SyntaxNode) -> SyntaxElement { NodeOrToken :: Node (node) } }
    };
}

impl_33!()