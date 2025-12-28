macro_rules! deps {
    () => {
        FloatTypeWrapper!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl fmt :: Display for FloatTypeWrapper { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . 0 . as_str ()) } }
    };
}

impl_221!();