macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > OutputSizeUser for SimpleHmac < D > { type OutputSize = D :: OutputSize ; }
    };
}

impl_31!();