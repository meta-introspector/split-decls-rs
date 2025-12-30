// Generated macro for impl_146 (impl)
macro_rules! Depcrate_oneshotimpl_146 {
() => {
// Module: crate::oneshot
// Provides: {"impl_146"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { self . inner . drop_tx () } }
};
}
