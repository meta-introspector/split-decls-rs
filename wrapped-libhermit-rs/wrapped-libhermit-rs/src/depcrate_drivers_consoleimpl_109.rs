// Generated macro for impl_109 (impl)
macro_rules! Depcrate_drivers_consoleimpl_109 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_109"}
// Dependencies: {}
impl Read for VirtioUART { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { if let Some (drv) = get_console_driver () { drv . lock () . read (buf) } else { Err (Errno :: Io) } } }
};
}
