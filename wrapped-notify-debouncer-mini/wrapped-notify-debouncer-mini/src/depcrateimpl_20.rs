// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : Watcher > Debouncer < T > { # [doc = " Access to the internally used notify Watcher backend"] pub fn watcher (& mut self) -> & mut dyn Watcher { & mut self . watcher } }
};
}
