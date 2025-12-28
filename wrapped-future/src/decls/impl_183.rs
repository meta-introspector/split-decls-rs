macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
        AsyncFuture!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IntoFuture for IAsyncOperationWithProgress < T , P > { type Output = Result < T > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
    };
}

impl_183!()