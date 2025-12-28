macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Display for OutOfRange { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "out of range") } }
    };
}

impl_18!()