macro_rules! deps {
    () => {
        SyntaxNode!();
        AstNode!();
        AstChildren!();
        Language!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < N : AstNode > AstChildren < N > { fn new (parent : & SyntaxNode < N :: Language >) -> Self { AstChildren { inner : parent . children () , ph : PhantomData } } }
    };
}

impl_183!();