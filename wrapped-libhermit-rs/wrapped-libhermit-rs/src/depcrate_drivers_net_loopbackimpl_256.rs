// Generated macro for impl_256 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_256 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_256"}
// Dependencies: {}
impl smoltcp :: phy :: RxToken for RxToken < '_ > { fn consume < R , F > (self , f : F) -> R where F : FnOnce (& [u8]) -> R , { let frame = self . queue . lock () . pop_front () ; f (& frame . unwrap ()) } }
};
}
