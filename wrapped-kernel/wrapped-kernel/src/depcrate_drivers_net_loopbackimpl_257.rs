// Generated macro for impl_257 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_257 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_257"}
// Dependencies: {}
impl Drop for RxToken < '_ > { fn drop (& mut self) { self . reserved_receives . fetch_sub (1 , Ordering :: Relaxed) ; } }
};
}
