// Generated macro for impl_229 (impl)
macro_rules! Depcrate_utils_channelimpl_229 {
() => {
// Module: crate::utils::channel
// Provides: {"impl_229"}
// Dependencies: {}
impl < T > LocalSender < T > { pub (crate) fn send (& self , item : T) { let mut channel = self . channel . borrow_mut () ; channel . queue . push_back (item) ; let _ = channel . waker . take () . map (Waker :: wake) ; } }
};
}
