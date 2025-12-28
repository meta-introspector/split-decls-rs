macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < N : AstNode > Clone for AstPtr < N > { fn clone (& self) -> Self { Self { raw : self . raw } } }
    };
}

impl_177!();