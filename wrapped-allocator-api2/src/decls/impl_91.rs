macro_rules! deps {
    () => {
        RawVec!();
        Allocator!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Sync for RawVec < T , A > where T : Sync , A : Sync , { }
    };
}

impl_91!();