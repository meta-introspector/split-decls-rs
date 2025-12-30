// Generated macro for macro_442 (macro)
macro_rules! Depcrate_drivers_netmacro_442 {
() => {
// Module: crate::drivers::net
// Provides: {"macro_442"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (all (not (feature = "pci") , any (all (target_arch = "riscv64" , feature = "gem-net") , feature = "virtio-net" ,) ,))] { pub (crate) use crate :: arch :: kernel :: mmio :: NetworkDevice ; } else if # [cfg (all (feature = "pci" , any (feature = "rtl8139" , feature = "virtio-net" ,) ,))] { pub (crate) use crate :: drivers :: pci :: NetworkDevice ; } else { pub (crate) use loopback :: NetworkDevice ; } }
};
}
