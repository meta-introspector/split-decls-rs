macro_rules! deps {
    () => {
        OperationWithProgress!();
        SyncState!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Creates an `IAsyncOperationWithProgress<T, P>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < T > + Send + 'static , { let object = ComObject :: new (OperationWithProgress (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
    };
}

impl_172!()