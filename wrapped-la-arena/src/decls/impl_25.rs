macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Display for RawIdx { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_25!();