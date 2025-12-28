macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for RawOccupiedEntryMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawOccupiedEntryMut") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
    };
}

impl_44!()