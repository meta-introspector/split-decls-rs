macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        unsafe impl < K , V , S , A > Send for RawOccupiedEntryMut < '_ , K , V , S , A > where K : Send , V : Send , S : Send , A : Send + Allocator , { }
    };
}

impl_347!()