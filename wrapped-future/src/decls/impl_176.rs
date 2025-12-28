macro_rules! deps {
    () => {
        AsyncFuture!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        unsafe impl < A : Async > Send for AsyncFuture < A > { }
    };
}

impl_176!()