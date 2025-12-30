// Generated macro for RxToken (struct)
macro_rules! Depcrate_drivers_net_loopbackRxToken {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"RxToken"}
// Dependencies: {}
pub (crate) struct RxToken < 'a > { queue : & 'a SpinMutex < VecDeque < Vec < u8 , DeviceAlloc > > > , reserved_receives : & 'a AtomicUsize , }
};
}
