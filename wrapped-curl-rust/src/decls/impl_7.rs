macro_rules! deps {
    () => {
        ShareError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for ShareError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . description () . fmt (f) } }
    };
}

impl_7!()