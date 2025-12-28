macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Send for RawTable < T , A > where T : Send , A : Send , { }
    };
}

impl_53!();