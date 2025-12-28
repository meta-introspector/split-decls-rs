macro_rules! deps {
    () => {
        Upgraded!();
        Result!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl fmt :: Debug for Upgraded { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Upgraded") . finish () } }
    };
}

impl_217!()