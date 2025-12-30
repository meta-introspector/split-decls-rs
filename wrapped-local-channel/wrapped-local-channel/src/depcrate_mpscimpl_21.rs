// Generated macro for impl_21 (impl)
macro_rules! Depcrate_mpscimpl_21 {
() => {
// Module: crate::mpsc
// Provides: {"impl_21"}
// Dependencies: {}
impl < T > Drop for Receiver < T > { fn drop (& mut self) { let mut shared = self . shared . borrow_mut () ; shared . buffer . clear () ; shared . has_receiver = false ; } }
};
}
