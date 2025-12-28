macro_rules! deps {
    () => {
        Result!();
        PathValue!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl fmt :: Debug for PathValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_188!()