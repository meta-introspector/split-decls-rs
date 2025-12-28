macro_rules! deps {
    () => {
        Condvar!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Debug for Condvar { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Condvar { .. }") } }
    };
}

impl_5!();