macro_rules! deps {
    () => {
        Language!();
    };
}

macro_rules! Preorder {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Preorder < L : Language > { raw : cursor :: Preorder , _p : PhantomData < L > , }
    };
}

Preorder!()