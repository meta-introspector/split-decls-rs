// Generated macro for impl_230 (impl)
macro_rules! Depcrate_utils_channelimpl_230 {
() => {
// Module: crate::utils::channel
// Provides: {"impl_230"}
// Dependencies: {}
impl < T > Drop for LocalSender < T > { fn drop (& mut self) { let mut channel = self . channel . borrow_mut () ; channel . closed = true ; let _ = channel . waker . take () . map (Waker :: wake) ; } }
};
}
