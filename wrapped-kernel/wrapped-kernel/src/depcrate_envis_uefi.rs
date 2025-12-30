// Generated macro for is_uefi (function)
macro_rules! Depcrate_envis_uefi {
() => {
// Module: crate::env
// Provides: {"is_uefi"}
// Dependencies: {}
pub fn is_uefi () -> bool { fdt () . is_some_and (| fdt | fdt . root () . compatible () . first () == "hermit,uefi") }
};
}
