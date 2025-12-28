macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . message) } }
    };
}

impl_6!()