macro_rules! deps {
    () => {
        ReadyOperationWithProgress!();
        ReadyState!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Creates an `IAsyncOperationWithProgress<T, P>` that is immediately ready with a value."] pub fn ready (result : Result < T >) -> Self { ReadyOperationWithProgress (ReadyState :: new (result)) . into () } }
    };
}

impl_150!();