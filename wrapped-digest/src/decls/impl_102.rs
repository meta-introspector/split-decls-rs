macro_rules! deps {
    () => {
        InvalidOutputSize!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl fmt :: Display for InvalidOutputSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("invalid output size") } }
    };
}

impl_102!();