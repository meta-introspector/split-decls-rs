macro_rules! deps {
    () => {
        RustcOccupiedEntry!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        unsafe impl < K , V , A > Send for RustcOccupiedEntry < '_ , K , V , A > where K : Send , V : Send , A : Allocator + Send , { }
    };
}

impl_368!()