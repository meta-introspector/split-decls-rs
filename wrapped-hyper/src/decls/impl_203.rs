macro_rules! deps {
    () => {
        ServiceFn!();
        Result!();
        Service!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < F , R > fmt :: Debug for ServiceFn < F , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("impl Service") . finish () } }
    };
}

impl_203!();