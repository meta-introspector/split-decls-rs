macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Waits for the `IAsyncOperationWithProgress<T, P>` to finish."] pub fn join (& self) -> Result < T > { Async :: join (self) } }
    };
}

impl_119!()