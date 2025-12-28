macro_rules! deps {
    () => {
        Allocator!();
        IntoIter!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        unsafe impl < T : Send , A : Allocator + Send > Send for IntoIter < T , A > { }
    };
}

impl_124!()