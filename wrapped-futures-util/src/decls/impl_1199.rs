macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_1199 {
    () => {
        deps!();
        impl fmt :: Debug for Sink { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Sink { .. }") } }
    };
}

impl_1199!()