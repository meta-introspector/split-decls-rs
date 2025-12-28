macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl crypto_common :: BlockSizeUser for Hasher { type BlockSize = U64 ; }
    };
}

impl_135!()