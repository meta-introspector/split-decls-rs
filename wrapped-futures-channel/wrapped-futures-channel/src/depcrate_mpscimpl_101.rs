// Generated macro for impl_101 (impl)
macro_rules! Depcrate_mpscimpl_101 {
() => {
// Module: crate::mpsc
// Provides: {"impl_101"}
// Dependencies: {}
impl < T > Drop for UnboundedSenderInner < T > { fn drop (& mut self) { let prev = self . inner . num_senders . fetch_sub (1 , SeqCst) ; if prev == 1 { self . close_channel () ; } } }
};
}
