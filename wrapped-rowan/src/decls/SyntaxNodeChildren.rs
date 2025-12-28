macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! SyntaxNodeChildren {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct SyntaxNodeChildren < L : Language > { raw : cursor :: SyntaxNodeChildren , _p : PhantomData < L > , }
    };
}

SyntaxNodeChildren!()