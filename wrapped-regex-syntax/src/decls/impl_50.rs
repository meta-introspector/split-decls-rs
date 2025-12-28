macro_rules! deps {
    () => {
        Error!();
        Formatter!();
        Result!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { crate :: error :: Formatter :: from (self) . fmt (f) } }
    };
}

impl_50!()