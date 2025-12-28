macro_rules! deps {
    () => {
        AsyncFuture!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        unsafe impl < A : Async > Sync for AsyncFuture < A > { }
    };
}

impl_177!();