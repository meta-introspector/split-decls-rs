macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl fmt :: Debug for RawIdx { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_24!()