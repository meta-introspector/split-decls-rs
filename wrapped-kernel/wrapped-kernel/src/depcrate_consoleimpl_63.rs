// Generated macro for impl_63 (impl)
macro_rules! Depcrate_consoleimpl_63 {
() => {
// Module: crate::console
// Provides: {"impl_63"}
// Dependencies: {}
impl Read for IoDevice { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { match self { # [cfg (not (target_arch = "riscv64"))] IoDevice :: Uhyve (s) => s . read (buf) , IoDevice :: Uart (s) => s . read (buf) , # [cfg (feature = "console")] IoDevice :: Virtio (s) => s . read (buf) , } } }
};
}
