// Generated macro for impl_102 (impl)
macro_rules! Depcrate_mpscimpl_102 {
() => {
// Module: crate::mpsc
// Provides: {"impl_102"}
// Dependencies: {}
impl < T > Drop for BoundedSenderInner < T > { fn drop (& mut self) { let prev = self . inner . num_senders . fetch_sub (1 , SeqCst) ; if prev == 1 { self . close_channel () ; } } }
};
}
