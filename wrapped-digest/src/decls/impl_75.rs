macro_rules! deps {
    () => {
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Default , S : ArraySize > Default for XofFixedWrapper < T , S > { fn default () -> Self { Self { hash : Default :: default () , size : PhantomData , } } }
    };
}

impl_75!();