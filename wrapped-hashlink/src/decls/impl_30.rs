macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V , S > fmt :: Debug for VacantEntry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
    };
}

impl_30!();