macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Calls `op(result)` when the `IAsyncActionWithProgress<P>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < () >) + Send + 'static , { Async :: when (self , op) } }
    };
}

impl_130!()