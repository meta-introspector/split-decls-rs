macro_rules! deps {
    () => {
        RawEntryBuilderMut!();
        RawEntryBuilder!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl < K , V , S , A : Allocator > Debug for RawEntryBuilderMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawEntryBuilder") . finish () } }
    };
}

impl_357!()