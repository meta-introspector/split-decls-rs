macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Debug for OutOfRange { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "out of range") } }
    };
}

impl_19!()