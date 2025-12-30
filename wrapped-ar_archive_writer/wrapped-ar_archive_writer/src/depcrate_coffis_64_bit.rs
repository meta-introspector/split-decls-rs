// Generated macro for is_64_bit (function)
macro_rules! Depcrate_coffis_64_bit {
() => {
// Module: crate::coff
// Provides: {"is_64_bit"}
// Dependencies: {}
pub fn is_64_bit (machine : MachineTypes) -> bool { machine == MachineTypes :: AMD64 || is_any_arm64 (machine) }
};
}
