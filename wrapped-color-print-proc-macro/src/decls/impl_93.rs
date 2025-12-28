macro_rules! deps {
    () => {
        ErrorDetail!();
        Result!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for ErrorDetail < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . message) } }
    };
}

impl_93!()