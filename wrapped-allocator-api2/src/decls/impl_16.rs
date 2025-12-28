macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        unsafe impl < T : ? Sized , A : Allocator > Send for Box < T , A > where T : Send , A : Send , { }
    };
}

impl_16!()