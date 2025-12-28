macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: EmptyRange => "low > high (or equal if exclusive) in uniform distribution" , Error :: NonFinite => "Non-finite range in uniform distribution" , }) } }
    };
}

impl_160!();