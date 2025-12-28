macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Eq for SyntaxNode { }
    };
}

impl_25!();