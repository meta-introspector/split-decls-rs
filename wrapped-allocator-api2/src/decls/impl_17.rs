macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        unsafe impl < T : ? Sized , A : Allocator > Sync for Box < T , A > where T : Sync , A : Sync , { }
    };
}

impl_17!();