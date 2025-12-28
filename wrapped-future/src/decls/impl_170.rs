macro_rules! deps {
    () => {
        IAsyncOperation!();
        Operation!();
        SyncState!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Creates an `IAsyncOperation<T>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < T > + Send + 'static , { let object = ComObject :: new (Operation (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
    };
}

impl_170!();