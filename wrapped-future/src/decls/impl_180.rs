macro_rules! deps {
    () => {
        AsyncFuture!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl IntoFuture for IAsyncAction { type Output = Result < () > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
    };
}

impl_180!()