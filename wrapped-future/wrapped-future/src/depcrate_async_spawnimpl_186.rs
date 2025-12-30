// Generated macro for impl_186 (impl)
macro_rules! Depcrate_async_spawnimpl_186 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_186"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Creates an `IAsyncOperationWithProgress<T, P>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < T > + Send + 'static , { let object = ComObject :: new (OperationWithProgress (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
};
}
