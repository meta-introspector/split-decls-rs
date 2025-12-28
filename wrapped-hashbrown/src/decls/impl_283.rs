macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        unsafe impl < K , V , S , A > Sync for OccupiedEntry < '_ , K , V , S , A > where K : Sync , V : Sync , S : Sync , A : Sync + Allocator , { }
    };
}

impl_283!();