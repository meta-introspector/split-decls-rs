macro_rules! deps {
    () => {
        ParallelSlice!();
    };
}

macro_rules! impl_1238 {
    () => {
        deps!();
        impl < T : Sync > ParallelSlice < T > for [T] { # [inline] fn as_parallel_slice (& self) -> & [T] { self } }
    };
}

impl_1238!();