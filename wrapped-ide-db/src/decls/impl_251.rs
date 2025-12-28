macro_rules! deps {
    () => {
        RootDatabase!();
        Result!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl fmt :: Debug for RootDatabase { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RootDatabase") . finish () } }
    };
}

impl_251!();