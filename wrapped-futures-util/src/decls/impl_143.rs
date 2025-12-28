macro_rules! deps {
    () => {
        TryFutureExt!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < Fut : ? Sized + TryFuture > TryFutureExt for Fut { }
    };
}

impl_143!()