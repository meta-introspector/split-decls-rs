macro_rules! deps {
    () => {
        MultiError!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Display for MultiError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . description () . fmt (f) } }
    };
}

impl_12!()