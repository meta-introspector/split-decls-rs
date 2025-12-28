macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser + fmt :: Debug > fmt :: Debug for SimpleHmacReset < D > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("SimpleResetHmac") } }
    };
}

impl_42!();