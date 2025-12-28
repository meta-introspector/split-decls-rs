macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
        SyntaxElement!();
        NodeOrToken!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxNode < L > > for SyntaxElement < L > { fn from (node : SyntaxNode < L >) -> SyntaxElement < L > { NodeOrToken :: Node (node) } }
    };
}

impl_60!()