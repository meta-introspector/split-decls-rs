macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl fmt :: Debug for Guard { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Guard { .. }") } }
    };
}

impl_96!()