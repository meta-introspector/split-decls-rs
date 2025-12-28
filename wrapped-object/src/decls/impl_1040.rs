macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_1040 {
    () => {
        deps!();
        impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . 0) } }
    };
}

impl_1040!()