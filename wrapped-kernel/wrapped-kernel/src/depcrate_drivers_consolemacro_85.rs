// Generated macro for macro_85 (macro)
macro_rules! Depcrate_drivers_consolemacro_85 {
() => {
// Module: crate::drivers::console
// Provides: {"macro_85"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "pci")] { mod pci ; } else { mod mmio ; } }
};
}
