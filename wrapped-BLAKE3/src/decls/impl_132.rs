macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl digest :: ExtendableOutputReset for Hasher { # [inline] fn finalize_xof_reset (& mut self) -> Self :: Reader { let reader = Hasher :: finalize_xof (self) ; self . reset () ; reader } }
    };
}

impl_132!();