// Generated macro for is_copy_instruction (function)
macro_rules! Depcrate_instructionis_copy_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_copy_instruction"}
// Dependencies: {}
pub fn is_copy_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 1 == instruction_data [0] }
};
}
