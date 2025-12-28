macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Waits for the `IAsyncActionWithProgress<P>` to finish."] pub fn join (& self) -> Result < () > { Async :: join (self) } }
    };
}

impl_118!();