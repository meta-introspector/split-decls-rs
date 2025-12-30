// Generated macro for impl_122 (impl)
macro_rules! Depcrate_drivers_consoleimpl_122 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_122"}
// Dependencies: {}
impl Write for VirtioConsoleDriver { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { self . send_vq . send_packet (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
