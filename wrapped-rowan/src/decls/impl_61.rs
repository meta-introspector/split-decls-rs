macro_rules! deps {
    () => {
        NodeOrToken!();
        Language!();
        SyntaxToken!();
        SyntaxElement!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxToken < L > > for SyntaxElement < L > { fn from (token : SyntaxToken < L >) -> SyntaxElement < L > { NodeOrToken :: Token (token) } }
    };
}

impl_61!();