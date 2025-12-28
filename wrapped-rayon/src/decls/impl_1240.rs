macro_rules! deps {
    () => {
        ParallelSliceMut!();
    };
}

macro_rules! impl_1240 {
    () => {
        deps!();
        impl < T : Send > ParallelSliceMut < T > for [T] { # [inline] fn as_parallel_slice_mut (& mut self) -> & mut [T] { self } }
    };
}

impl_1240!();