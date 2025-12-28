macro_rules! deps {
    () => {
        IAsyncOperation!();
        ReadyOperation!();
        ReadyState!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Creates an `IAsyncOperation<T>` that is immediately ready with a value."] pub fn ready (result : Result < T >) -> Self { ReadyOperation (ReadyState :: new (result)) . into () } }
    };
}

impl_148!()