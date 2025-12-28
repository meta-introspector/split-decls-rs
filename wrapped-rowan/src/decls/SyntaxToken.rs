macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! SyntaxToken {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash)] pub struct SyntaxToken < L : Language > { raw : cursor :: SyntaxToken , _p : PhantomData < L > , }
    };
}

SyntaxToken!();