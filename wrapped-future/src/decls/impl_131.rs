macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Calls `op(result)` when the `IAsyncOperationWithProgress<T, P>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < T >) + Send + 'static , { Async :: when (self , op) } }
    };
}

impl_131!();