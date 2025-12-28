macro_rules! deps {
    () => {
        AsyncFuture!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < A : Async > AsyncFuture < A > { fn new (inner : A) -> Self { Self { status : inner . cast () . unwrap () , inner , waker : None , } } }
    };
}

impl_175!();