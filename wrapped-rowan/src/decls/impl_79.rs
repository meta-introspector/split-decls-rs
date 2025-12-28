macro_rules! deps {
    () => {
        Language!();
        SyntaxToken!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < L : Language > From < cursor :: SyntaxToken > for SyntaxToken < L > { fn from (raw : cursor :: SyntaxToken) -> SyntaxToken < L > { SyntaxToken { raw , _p : PhantomData } } }
    };
}

impl_79!();