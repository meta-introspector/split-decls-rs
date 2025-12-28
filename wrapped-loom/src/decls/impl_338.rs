macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for JoinHandle < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("JoinHandle") . finish () } }
    };
}

impl_338!();