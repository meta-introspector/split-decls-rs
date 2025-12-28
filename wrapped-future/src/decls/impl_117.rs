macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Waits for the `IAsyncOperation<T>` to finish."] pub fn join (& self) -> Result < T > { Async :: join (self) } }
    };
}

impl_117!();