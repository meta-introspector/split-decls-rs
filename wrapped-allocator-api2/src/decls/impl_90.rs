macro_rules! deps {
    () => {
        RawVec!();
        Allocator!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Send for RawVec < T , A > where T : Send , A : Send , { }
    };
}

impl_90!()