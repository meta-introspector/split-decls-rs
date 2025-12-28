macro_rules! deps {
    () => {
        FormError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl fmt :: Display for FormError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . description () . fmt (f) } }
    };
}

impl_17!()