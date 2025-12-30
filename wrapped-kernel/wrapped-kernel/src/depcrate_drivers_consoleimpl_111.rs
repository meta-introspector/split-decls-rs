// Generated macro for impl_111 (impl)
macro_rules! Depcrate_drivers_consoleimpl_111 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_111"}
// Dependencies: {}
impl Write for VirtioUART { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { if let Some (drv) = get_console_driver () { drv . lock () . write_all (buf) ? ; } Ok (buf . len ()) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
