macro_rules! deps {
    () => {
        RawEntryBuilder!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for RawEntryBuilder < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawEntryBuilder") . finish () } }
    };
}

impl_46!()