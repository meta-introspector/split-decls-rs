macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxNode < L > > for cursor :: SyntaxNode { fn from (node : SyntaxNode < L >) -> cursor :: SyntaxNode { node . raw } }
    };
}

impl_78!()