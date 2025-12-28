macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T : ExtendableOutput + BlockSizeUser , S : ArraySize > BlockSizeUser for XofFixedWrapper < T , S > { type BlockSize = T :: BlockSize ; }
    };
}

impl_79!();