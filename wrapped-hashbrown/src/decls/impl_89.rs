macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Send for RawIntoIter < T , A > where T : Send , A : Send , { }
    };
}

impl_89!();