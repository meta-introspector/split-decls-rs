macro_rules! deps {
    () => {
        ByteString!();
        Result!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'data > fmt :: Debug for ByteString < 'data > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "\"{}\"" , String :: from_utf8_lossy (self . 0)) } }
    };
}

impl_107!()