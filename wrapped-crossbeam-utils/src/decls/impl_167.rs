macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T > fmt :: Debug for ScopedJoinHandle < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("ScopedJoinHandle { .. }") } }
    };
}

impl_167!()