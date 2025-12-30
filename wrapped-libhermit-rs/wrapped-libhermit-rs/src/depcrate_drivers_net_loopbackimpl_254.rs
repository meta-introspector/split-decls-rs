// Generated macro for impl_254 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_254 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_254"}
// Dependencies: {}
impl smoltcp :: phy :: TxToken for TxToken < '_ > { fn consume < R , F > (self , len : usize , f : F) -> R where F : FnOnce (& mut [u8]) -> R , { let mut buffer = Vec :: with_capacity_in (len , DeviceAlloc) ; buffer . resize (len , 0) ; let result = f (& mut buffer) ; self . queue . lock () . push_back (buffer) ; result } }
};
}
