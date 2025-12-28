macro_rules! deps {
    () => {
        Formatter!();
        Error!();
        Result!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& * self . err , f) } }
    };
}

impl_65!();