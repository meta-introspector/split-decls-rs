macro_rules! deps {
    () => {
        RegionName!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Display for RegionName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name) } }
    };
}

impl_84!();