macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl fmt :: Debug for RootDatabase { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RootDatabase") . finish () } }
    };
}

impl_33!()