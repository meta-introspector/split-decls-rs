// Generated macro for is_upgrade_instruction (function)
macro_rules! Depcrate_instructionis_upgrade_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_upgrade_instruction"}
// Dependencies: {}
pub fn is_upgrade_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 3 == instruction_data [0] }
};
}
