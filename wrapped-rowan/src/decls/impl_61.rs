macro_rules! deps {
    () => {
        SyntaxElement!();
        Language!();
        NodeOrToken!();
        SyntaxToken!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxToken < L > > for SyntaxElement < L > { fn from (token : SyntaxToken < L >) -> SyntaxElement < L > { NodeOrToken :: Token (token) } }
    };
}

impl_61!()