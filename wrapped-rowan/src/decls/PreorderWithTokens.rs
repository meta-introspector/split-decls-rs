macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! PreorderWithTokens {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct PreorderWithTokens < L : Language > { raw : cursor :: PreorderWithTokens , _p : PhantomData < L > , }
    };
}

PreorderWithTokens!()