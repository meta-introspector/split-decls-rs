macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        unsafe impl < K , V , S , A > Send for OccupiedEntry < '_ , K , V , S , A > where K : Send , V : Send , S : Send , A : Send + Allocator , { }
    };
}

impl_282!();