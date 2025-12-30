// Generated macro for impl_259 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_259 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_259"}
// Dependencies: {}
impl NetworkDriver for LoopbackDriver { fn get_mac_address (& self) -> [u8 ; 6] { [0 ; 6] } fn has_packet (& self) -> bool { ! self . queue . lock () . is_empty () } fn set_polling_mode (& mut self , _value : bool) { } fn handle_interrupt (& mut self) { } }
};
}
