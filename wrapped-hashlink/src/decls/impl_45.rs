macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for RawVacantEntryMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawVacantEntryMut") . finish () } }
    };
}

impl_45!();