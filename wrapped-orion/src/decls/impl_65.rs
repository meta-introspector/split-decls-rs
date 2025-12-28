macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Debug for UnknownCryptoError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "UnknownCryptoError") } }
    };
}

impl_65!();