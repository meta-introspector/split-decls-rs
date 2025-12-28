macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
        AsyncFuture!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < P : RuntimeType > IntoFuture for IAsyncActionWithProgress < P > { type Output = Result < () > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
    };
}

impl_182!();