macro_rules! deps {
    () => {
        Formatter!();
        Buffer!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Buffer { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { String :: from_utf8_lossy (self . as_bytes ()) . fmt (f) } }
    };
}

impl_28!();