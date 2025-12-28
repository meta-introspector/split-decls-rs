macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > KeySizeUser for SimpleHmacReset < D > { type KeySize = D :: BlockSize ; }
    };
}

impl_36!()