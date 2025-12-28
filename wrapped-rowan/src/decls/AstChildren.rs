macro_rules! deps {
    () => {
        SyntaxNodeChildren!();
        Language!();
        AstNode!();
    };
}

macro_rules! AstChildren {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct AstChildren < N : AstNode > { inner : SyntaxNodeChildren < N :: Language > , ph : PhantomData < N > , }
    };
}

AstChildren!()