macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > KeySizeUser for SimpleHmac < D > { type KeySize = D :: BlockSize ; }
    };
}

impl_27!();