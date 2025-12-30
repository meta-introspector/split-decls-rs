// Generated macro for impl_433 (impl)
macro_rules! Depcrate_sync_mpscimpl_433 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_433"}
// Dependencies: {}
impl < T > Drop for Receiver < T > { fn drop (& mut self) { while ! self . object . is_empty () { self . recv () . unwrap () ; } } }
};
}
