macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > OutputSizeUser for SimpleHmacReset < D > { type OutputSize = D :: OutputSize ; }
    };
}

impl_40!();