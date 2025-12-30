// Generated macro for impl_184 (impl)
macro_rules! Depcrate_async_spawnimpl_184 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_184"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Creates an `IAsyncOperation<T>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < T > + Send + 'static , { let object = ComObject :: new (Operation (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
};
}
