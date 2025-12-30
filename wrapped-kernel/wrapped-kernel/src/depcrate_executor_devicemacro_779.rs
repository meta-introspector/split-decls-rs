// Generated macro for macro_779 (macro)
macro_rules! Depcrate_executor_devicemacro_779 {
() => {
// Module: crate::executor::device
// Provides: {"macro_779"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci")) , feature = "rtl8139" , feature = "virtio-net" ,))] { use hermit_sync :: SpinMutex ; use crate :: drivers :: net :: NetworkDevice ; pub (crate) static NETWORK_DEVICE : SpinMutex < Option < NetworkDevice >> = SpinMutex :: new (Option :: None) ; } else { use crate :: drivers :: net :: loopback :: LoopbackDriver ; } }
};
}
