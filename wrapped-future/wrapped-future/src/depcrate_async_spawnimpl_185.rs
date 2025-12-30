// Generated macro for impl_185 (impl)
macro_rules! Depcrate_async_spawnimpl_185 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_185"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Creates an `IAsyncActionWithProgress<P>` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < () > + Send + 'static , { let object = ComObject :: new (ActionWithProgress (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
};
}
