macro_rules! deps {
    () => {
        Drain!();
        Allocator!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        unsafe impl < T : Sync , A : Sync + Allocator > Sync for Drain < '_ , T , A > { }
    };
}

impl_112!()