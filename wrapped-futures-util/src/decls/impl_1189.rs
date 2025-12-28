macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_1189 {
    () => {
        deps!();
        impl fmt :: Debug for Repeat { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Repeat { .. }") } }
    };
}

impl_1189!();