macro_rules! deps {
    () => {
        NodeOrToken!();
        SyntaxToken!();
        SyntaxElement!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl From < SyntaxToken > for SyntaxElement { # [inline] fn from (token : SyntaxToken) -> SyntaxElement { NodeOrToken :: Token (token) } }
    };
}

impl_34!();