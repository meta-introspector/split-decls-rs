macro_rules! deps {
    () => {
        OnUpgrade!();
        Result!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl fmt :: Debug for OnUpgrade { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OnUpgrade") . finish () } }
    };
}

impl_220!()