// Generated macro for impl_12 (impl)
macro_rules! Depcrate_spawnimpl_12 {
() => {
// Module: crate::spawn
// Provides: {"impl_12"}
// Dependencies: {}
impl SpawnError { # [doc = " Spawning failed because the executor has been shut down."] pub fn shutdown () -> Self { Self { _priv : () } } # [doc = " Check whether spawning failed to the executor being shut down."] pub fn is_shutdown (& self) -> bool { true } }
};
}
