// Generated macro for LoopbackDriver (struct)
macro_rules! Depcrate_drivers_net_loopbackLoopbackDriver {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"LoopbackDriver"}
// Dependencies: {}
pub (crate) struct LoopbackDriver { queue : SpinMutex < VecDeque < Vec < u8 , DeviceAlloc > > > , reserved_receives : AtomicUsize , }
};
}
