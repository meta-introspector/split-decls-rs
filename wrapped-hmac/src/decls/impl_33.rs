macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser + fmt :: Debug > fmt :: Debug for SimpleHmac < D > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("SimpleHmac { ... }") } }
    };
}

impl_33!();