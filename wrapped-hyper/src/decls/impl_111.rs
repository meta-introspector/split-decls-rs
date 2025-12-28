macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . description ()) } }
    };
}

impl_111!();