macro_rules! deps {
    () => {
        RawParDrain!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        unsafe impl < T : Send , A : Allocator > Send for RawParDrain < '_ , T , A > { }
    };
}

impl_171!();