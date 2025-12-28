macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        unsafe impl < T , A > Send for OccupiedEntry < '_ , T , A > where T : Send , A : Send + Allocator , { }
    };
}

impl_481!()