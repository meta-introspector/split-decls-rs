macro_rules! deps {
    () => {
        IAsyncOperation!();
        AsyncFuture!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T : RuntimeType > IntoFuture for IAsyncOperation < T > { type Output = Result < T > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
    };
}

impl_181!();