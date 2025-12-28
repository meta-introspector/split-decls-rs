macro_rules! deps {
    () => {
        Unparker!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl fmt :: Debug for Unparker { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Unparker { .. }") } }
    };
}

impl_103!();