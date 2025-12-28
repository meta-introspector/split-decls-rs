macro_rules! deps {
    () => {
        Result!();
        ByteString!();
    };
}

macro_rules! impl_1090 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for ByteString < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "\"{}\"" , String :: from_utf8_lossy (& self . 0)) } }
    };
}

impl_1090!()