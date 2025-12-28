macro_rules! deps {
    () => {
        RustcOccupiedEntry!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        unsafe impl < K , V , A > Sync for RustcOccupiedEntry < '_ , K , V , A > where K : Sync , V : Sync , A : Allocator + Sync , { }
    };
}

impl_369!();