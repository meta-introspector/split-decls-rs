macro_rules! deps {
    () => {
        RegionSnapshot!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl fmt :: Debug for RegionSnapshot { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "RegionSnapshot") } }
    };
}

impl_141!();