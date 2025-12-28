macro_rules! deps {
    () => {
        ExtendableOutputReset!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T : ExtendableOutputReset , S : ArraySize > ExtendableOutputReset for XofFixedWrapper < T , S > { fn finalize_xof_reset (& mut self) -> Self :: Reader { self . hash . finalize_xof_reset () } }
    };
}

impl_88!();