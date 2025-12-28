macro_rules! deps {
    () => {
        Result!();
        OnUpgrade!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl fmt :: Debug for OnUpgrade { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OnUpgrade") . finish () } }
    };
}

impl_220!();