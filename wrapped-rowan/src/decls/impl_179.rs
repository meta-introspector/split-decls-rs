macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < N : AstNode > Eq for AstPtr < N > { }
    };
}

impl_179!();