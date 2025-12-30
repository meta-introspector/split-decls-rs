// Generated macro for macro_389 (macro)
macro_rules! Depcrate_drivers_net_virtiomacro_389 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"macro_389"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "pci")] { mod pci ; } else { mod mmio ; } }
};
}
