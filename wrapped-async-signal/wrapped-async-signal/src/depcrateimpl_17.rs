// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Drop for Signals { fn drop (& mut self) { for signal in self . signal_ids . values () { registry :: unregister (* signal) ; } } }
};
}
