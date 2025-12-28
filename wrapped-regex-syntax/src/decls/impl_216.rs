macro_rules! deps {
    () => {
        Formatter!();
        Error!();
        Result!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { crate :: error :: Formatter :: from (self) . fmt (f) } }
    };
}

impl_216!()