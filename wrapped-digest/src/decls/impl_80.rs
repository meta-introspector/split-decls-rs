macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T : ExtendableOutput + KeySizeUser , S : ArraySize > KeySizeUser for XofFixedWrapper < T , S > { type KeySize = T :: KeySize ; }
    };
}

impl_80!();