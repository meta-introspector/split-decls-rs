// Generated macro for impl_65 (impl)
macro_rules! Depcrate_consoleimpl_65 {
() => {
// Module: crate::console
// Provides: {"impl_65"}
// Dependencies: {}
impl Write for IoDevice { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { match self { # [cfg (not (target_arch = "riscv64"))] IoDevice :: Uhyve (s) => s . write_all (buf) ? , IoDevice :: Uart (s) => s . write_all (buf) ? , # [cfg (feature = "console")] IoDevice :: Virtio (s) => s . write_all (buf) ? , } ; # [cfg (all (target_arch = "x86_64" , feature = "vga"))] for & byte in buf { crate :: arch :: kernel :: vga :: write_byte (byte) ; } Ok (buf . len ()) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
