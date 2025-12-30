// Generated macro for impl_110 (impl)
macro_rules! Depcrate_drivers_consoleimpl_110 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_110"}
// Dependencies: {}
impl ReadReady for VirtioUART { fn read_ready (& mut self) -> Result < bool , Self :: Error > { if let Some (drv) = get_console_driver () { Ok (drv . lock () . has_packet ()) } else { Ok (false) } } }
};
}
