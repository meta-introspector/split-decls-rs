macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_882 {
    () => {
        deps!();
        impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . 0) } }
    };
}

impl_882!();