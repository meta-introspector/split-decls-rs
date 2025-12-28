macro_rules! deps {
    () => {
        SyntaxToken!();
        Language!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxToken < L > > for cursor :: SyntaxToken { fn from (token : SyntaxToken < L >) -> cursor :: SyntaxToken { token . raw } }
    };
}

impl_80!()