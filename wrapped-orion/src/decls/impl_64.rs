macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl fmt :: Display for UnknownCryptoError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "UnknownCryptoError") } }
    };
}

impl_64!()