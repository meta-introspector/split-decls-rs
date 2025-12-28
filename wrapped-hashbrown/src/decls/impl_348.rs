macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        unsafe impl < K , V , S , A > Sync for RawOccupiedEntryMut < '_ , K , V , S , A > where K : Sync , V : Sync , S : Sync , A : Sync + Allocator , { }
    };
}

impl_348!();