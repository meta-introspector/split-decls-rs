// Generated macro for impl_251 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_251 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_251"}
// Dependencies: {}
impl LoopbackDriver { pub (crate) const fn new () -> Self { Self { queue : SpinMutex :: new (VecDeque :: new ()) , reserved_receives : AtomicUsize :: new (0) , } } }
};
}
