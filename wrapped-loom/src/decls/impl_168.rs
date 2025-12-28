macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl fmt :: Display for Id { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . id . fmt (fmt) } }
    };
}

impl_168!();