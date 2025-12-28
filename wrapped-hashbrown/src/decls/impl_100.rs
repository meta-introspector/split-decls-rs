macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        unsafe impl < T , A : Allocator > Sync for RawDrain < '_ , T , A > where T : Sync , A : Sync , { }
    };
}

impl_100!()