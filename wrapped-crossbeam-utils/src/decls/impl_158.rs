macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl fmt :: Debug for Scope < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Scope { .. }") } }
    };
}

impl_158!()