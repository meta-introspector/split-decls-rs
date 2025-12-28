macro_rules! deps {
    () => {
        FixedOutputReset!();
        XofFixedWrapper!();
        ExtendableOutputReset!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T : ExtendableOutputReset , S : ArraySize > FixedOutputReset for XofFixedWrapper < T , S > { fn finalize_into_reset (& mut self , out : & mut crypto_common :: Output < Self >) { self . hash . finalize_xof_reset_into (out) ; } }
    };
}

impl_86!()