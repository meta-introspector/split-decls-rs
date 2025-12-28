macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Eq for SyntaxToken { }
    };
}

impl_30!();