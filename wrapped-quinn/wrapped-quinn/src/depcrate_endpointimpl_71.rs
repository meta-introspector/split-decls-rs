// Generated macro for impl_71 (impl)
macro_rules! Depcrate_endpointimpl_71 {
() => {
// Module: crate::endpoint
// Provides: {"impl_71"}
// Dependencies: {}
impl Drop for EndpointDriver { fn drop (& mut self) { let mut endpoint = self . 0 . state . lock () . unwrap () ; endpoint . driver_lost = true ; self . 0 . shared . incoming . notify_waiters () ; endpoint . recv_state . connections . senders . clear () ; } }
};
}
