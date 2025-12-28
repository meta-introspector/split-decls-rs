macro_rules! deps {
    () => {
        ReadyActionWithProgress!();
        ReadyState!();
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Creates an `IAsyncActionWithProgress<P>` that is immediately ready with a value."] pub fn ready (result : Result < () >) -> Self { ReadyActionWithProgress (ReadyState :: new (result)) . into () } }
    };
}

impl_149!();