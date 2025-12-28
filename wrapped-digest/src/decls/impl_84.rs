macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T : ExtendableOutput , S : ArraySize > OutputSizeUser for XofFixedWrapper < T , S > { type OutputSize = S ; }
    };
}

impl_84!();