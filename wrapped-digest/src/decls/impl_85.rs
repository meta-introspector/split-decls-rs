macro_rules! deps {
    () => {
        ExtendableOutput!();
        Update!();
        XofFixedWrapper!();
        FixedOutput!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Update , S : ArraySize > FixedOutput for XofFixedWrapper < T , S > { fn finalize_into (self , out : & mut crypto_common :: Output < Self >) { self . hash . finalize_xof_into (out) ; } }
    };
}

impl_85!();