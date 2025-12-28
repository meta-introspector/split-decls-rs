macro_rules! deps {
    () => {
        Injector!();
        Worker!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Injector < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Worker { .. }") } }
    };
}

impl_40!();