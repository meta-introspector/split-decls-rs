// Generated macro for impl_183 (impl)
macro_rules! Depcrate_async_spawnimpl_183 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_183"}
// Dependencies: {}
impl IAsyncAction { # [doc = " Creates an `IAsyncAction` that waits for the closure to execute on the Windows thread pool."] pub fn spawn < F > (f : F) -> Self where F : FnOnce () -> Result < () > + Send + 'static , { let object = ComObject :: new (Action (SyncState :: new ())) ; let interface = object . to_interface () ; windows_threading :: submit (move | | { object . 0 . spawn (& object . as_interface () , f) ; }) ; interface } }
};
}
