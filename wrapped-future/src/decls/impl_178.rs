macro_rules! deps {
    () => {
        AsyncFuture!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < A : Async > Unpin for AsyncFuture < A > { }
    };
}

impl_178!();