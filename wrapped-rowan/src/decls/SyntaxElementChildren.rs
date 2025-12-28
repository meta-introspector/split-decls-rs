macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! SyntaxElementChildren {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct SyntaxElementChildren < L : Language > { raw : cursor :: SyntaxElementChildren , _p : PhantomData < L > , }
    };
}

SyntaxElementChildren!();