macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl fmt :: Display for Level { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_str () . fmt (f) } }
    };
}

impl_52!()