macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        unsafe impl < T : Sync , A : Allocator + Sync > Sync for IntoIter < T , A > { }
    };
}

impl_125!()