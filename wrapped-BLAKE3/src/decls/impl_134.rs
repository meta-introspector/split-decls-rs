macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl crypto_common :: KeySizeUser for Hasher { type KeySize = U32 ; }
    };
}

impl_134!();