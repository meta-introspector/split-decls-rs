macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_497!()