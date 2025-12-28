macro_rules! deps {
    () => {
        Allocator!();
        Drain!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        unsafe impl < T : Send , A : Send + Allocator > Send for Drain < '_ , T , A > { }
    };
}

impl_113!()