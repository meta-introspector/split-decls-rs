macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl fmt :: Debug for Empty { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Empty { .. }") } }
    };
}

impl_237!();