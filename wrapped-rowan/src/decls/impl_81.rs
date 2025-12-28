macro_rules! deps {
    () => {
        Language!();
        SyntaxElement!();
        NodeOrToken!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < L : Language > From < cursor :: SyntaxElement > for SyntaxElement < L > { fn from (raw : cursor :: SyntaxElement) -> SyntaxElement < L > { match raw { NodeOrToken :: Node (it) => NodeOrToken :: Node (it . into ()) , NodeOrToken :: Token (it) => NodeOrToken :: Token (it . into ()) , } } }
    };
}

impl_81!()