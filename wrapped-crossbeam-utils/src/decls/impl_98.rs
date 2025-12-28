macro_rules! deps {
    () => {
        Parker!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl fmt :: Debug for Parker { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Parker { .. }") } }
    };
}

impl_98!()