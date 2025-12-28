macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Sync for RawIntoIter < T , A > where T : Sync , A : Sync , { }
    };
}

impl_90!()