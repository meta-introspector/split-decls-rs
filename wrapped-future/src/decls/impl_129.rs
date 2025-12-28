macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Calls `op(result)` when the `IAsyncOperation<T>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < T >) + Send + 'static , { Async :: when (self , op) } }
    };
}

impl_129!();