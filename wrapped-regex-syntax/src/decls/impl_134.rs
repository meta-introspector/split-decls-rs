macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        Error!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { Error :: Parse (ref x) => x . fmt (f) , Error :: Translate (ref x) => x . fmt (f) , } } }
    };
}

impl_134!();