macro_rules! deps {
    () => {
        Language!();
        SyntaxElement!();
        NodeOrToken!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < L : Language > From < SyntaxElement < L > > for cursor :: SyntaxElement { fn from (element : SyntaxElement < L >) -> cursor :: SyntaxElement { match element { NodeOrToken :: Node (it) => NodeOrToken :: Node (it . into ()) , NodeOrToken :: Token (it) => NodeOrToken :: Token (it . into ()) , } } }
    };
}

impl_82!();