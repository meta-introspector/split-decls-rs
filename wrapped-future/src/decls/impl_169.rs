macro_rules! deps {
    () => {
        Action!();
        SyncState!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl IAsyncAction { # [doc = " Creates an `IAsyncAction` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < () > + Send + 'static , { let object = ComObject :: new (Action (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
    };
}

impl_169!()