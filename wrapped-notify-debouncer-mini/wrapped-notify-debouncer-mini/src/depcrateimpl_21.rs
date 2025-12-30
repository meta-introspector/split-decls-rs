// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : Watcher > Drop for Debouncer < T > { fn drop (& mut self) { let _ = self . stop_channel . send (InnerEvent :: Shutdown) ; } }
};
}
