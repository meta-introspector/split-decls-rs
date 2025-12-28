macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < K , V , S , A : Allocator > Debug for RawVacantEntryMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawVacantEntryMut") . finish () } }
    };
}

impl_360!();