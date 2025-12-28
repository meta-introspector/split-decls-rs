macro_rules! deps {
    () => {
        Worker!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Worker < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Worker { .. }") } }
    };
}

impl_15!()