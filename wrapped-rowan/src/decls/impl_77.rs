macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < L : Language > From < cursor :: SyntaxNode > for SyntaxNode < L > { fn from (raw : cursor :: SyntaxNode) -> SyntaxNode < L > { SyntaxNode { raw , _p : PhantomData } } }
    };
}

impl_77!()