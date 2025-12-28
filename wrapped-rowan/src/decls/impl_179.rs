macro_rules! deps {
    () => {
        AstPtr!();
        AstNode!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < N : AstNode > Eq for AstPtr < N > { }
    };
}

impl_179!()