macro_rules! deps {
    () => {
        Language!();
        AstNode!();
        SyntaxNodeChildren!();
    };
}

macro_rules! AstChildren {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct AstChildren < N : AstNode > { inner : SyntaxNodeChildren < N :: Language > , ph : PhantomData < N > , }
    };
}

AstChildren!();