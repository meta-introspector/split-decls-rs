macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! SyntaxNode {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash)] pub struct SyntaxNode < L : Language > { raw : cursor :: SyntaxNode , _p : PhantomData < L > , }
    };
}

SyntaxNode!();