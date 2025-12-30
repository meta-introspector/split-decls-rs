// Generated macro for OnceLock (struct)
macro_rules! Depcrate_sync_once_lockOnceLock {
() => {
// Module: crate::sync::once_lock
// Provides: {"OnceLock"}
// Dependencies: {}
pub (crate) struct OnceLock < T > { once : Once , value : UnsafeCell < MaybeUninit < T > > , }
};
}
