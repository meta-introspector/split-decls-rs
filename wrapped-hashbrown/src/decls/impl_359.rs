macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > Debug for RawOccupiedEntryMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawOccupiedEntryMut") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
    };
}

impl_359!()