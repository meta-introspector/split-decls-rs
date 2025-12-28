macro_rules! deps {
    () => {
        DotDotPos!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl fmt :: Debug for DotDotPos { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_opt_usize () . fmt (f) } }
    };
}

impl_188!();