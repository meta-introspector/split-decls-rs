// Generated macro for is_any_arm64 (function)
macro_rules! Depcrate_coffis_any_arm64 {
() => {
// Module: crate::coff
// Provides: {"is_any_arm64"}
// Dependencies: {}
pub fn is_any_arm64 (machine : MachineTypes) -> bool { machine == MachineTypes :: ARM64 || is_arm64ec (machine) }
};
}
