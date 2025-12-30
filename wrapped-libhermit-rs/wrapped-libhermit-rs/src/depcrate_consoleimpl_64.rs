// Generated macro for impl_64 (impl)
macro_rules! Depcrate_consoleimpl_64 {
() => {
// Module: crate::console
// Provides: {"impl_64"}
// Dependencies: {}
impl ReadReady for IoDevice { fn read_ready (& mut self) -> Result < bool , Self :: Error > { match self { # [cfg (not (target_arch = "riscv64"))] IoDevice :: Uhyve (s) => s . read_ready () , IoDevice :: Uart (s) => s . read_ready () , # [cfg (feature = "console")] IoDevice :: Virtio (s) => s . read_ready () , } } }
};
}
