macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Send for RawDrain < '_ , T , A > where T : Send , A : Send , { }
    };
}

impl_99!()