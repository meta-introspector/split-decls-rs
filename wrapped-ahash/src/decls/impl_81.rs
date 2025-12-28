macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl fmt :: Debug for RandomState { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("RandomState { .. }") } }
    };
}

impl_81!();