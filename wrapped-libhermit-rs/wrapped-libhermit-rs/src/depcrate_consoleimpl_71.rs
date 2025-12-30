// Generated macro for impl_71 (impl)
macro_rules! Depcrate_consoleimpl_71 {
() => {
// Module: crate::console
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (not (target_arch = "riscv64"))] impl Write for UhyveSerial { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { serial_buf_hypercall (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
