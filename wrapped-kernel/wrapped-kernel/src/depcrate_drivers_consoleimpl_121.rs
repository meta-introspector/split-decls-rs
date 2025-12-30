// Generated macro for impl_121 (impl)
macro_rules! Depcrate_drivers_consoleimpl_121 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_121"}
// Dependencies: {}
impl Read for VirtioConsoleDriver { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { self . recv_vq . process_packet (| src | { buf [.. src . len ()] . copy_from_slice (src) ; src . len () }) . map_err (| _ | Errno :: Io) } }
};
}
