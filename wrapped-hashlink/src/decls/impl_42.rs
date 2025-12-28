macro_rules! deps {
    () => {
        RawEntryBuilderMut!();
        RawEntryBuilder!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for RawEntryBuilderMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawEntryBuilder") . finish () } }
    };
}

impl_42!()