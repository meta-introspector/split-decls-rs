macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
        SyncState!();
        ActionWithProgress!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Creates an `IAsyncActionWithProgress<P>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < () > + Send + 'static , { let object = ComObject :: new (ActionWithProgress (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
    };
}

impl_171!();