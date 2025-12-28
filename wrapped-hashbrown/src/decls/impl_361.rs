macro_rules! deps {
    () => {
        RawEntryBuilder!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < K , V , S , A : Allocator > Debug for RawEntryBuilder < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawEntryBuilder") . finish () } }
    };
}

impl_361!();