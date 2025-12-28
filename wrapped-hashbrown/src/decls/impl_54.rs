macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Sync for RawTable < T , A > where T : Sync , A : Sync , { }
    };
}

impl_54!();