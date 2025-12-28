macro_rules! deps {
    () => {
        ReadBuf!();
        Result!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl fmt :: Debug for ReadBuf < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuf") . field ("filled" , & self . filled) . field ("init" , & self . init) . field ("capacity" , & self . capacity ()) . finish () } }
    };
}

impl_169!()