macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Display for WildStr < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . line) } }
    };
}

impl_18!();