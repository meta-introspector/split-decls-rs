macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl digest :: OutputSizeUser for Hasher { type OutputSize = U32 ; }
    };
}

impl_128!();