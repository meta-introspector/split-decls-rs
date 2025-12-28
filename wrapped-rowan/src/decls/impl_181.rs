macro_rules! deps {
    () => {
        Language!();
        SyntaxNodePtr!();
        AstPtr!();
        AstNode!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < N : AstNode > From < AstPtr < N > > for SyntaxNodePtr < N :: Language > { fn from (ptr : AstPtr < N >) -> SyntaxNodePtr < N :: Language > { ptr . raw } }
    };
}

impl_181!()