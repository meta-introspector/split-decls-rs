macro_rules! deps {
    () => {
        Hasher!();
        OutputReader!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl digest :: ExtendableOutput for Hasher { type Reader = OutputReader ; # [inline] fn finalize_xof (self) -> Self :: Reader { Hasher :: finalize_xof (& self) } }
    };
}

impl_131!();