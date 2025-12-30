// Generated macro for impl_442 (impl)
macro_rules! Depcrate_sync_muteximpl_442 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_442"}
// Dependencies: {}
impl < T : ? Sized + Default > Default for Mutex < T > { # [doc = " Creates a `Mutex<T>`, with the `Default` value for T."] fn default () -> Self { Self :: new (Default :: default ()) } }
};
}
