macro_rules! deps {
    () => {
        LocalHandle!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl fmt :: Debug for LocalHandle { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("LocalHandle { .. }") } }
    };
}

impl_78!();