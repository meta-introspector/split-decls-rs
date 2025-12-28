macro_rules! deps {
    () => {
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < T : ExtendableOutput , S : ArraySize > ExtendableOutput for XofFixedWrapper < T , S > { type Reader = T :: Reader ; fn finalize_xof (self) -> Self :: Reader { self . hash . finalize_xof () } }
    };
}

impl_87!();