// Generated macro for is_write_instruction (function)
macro_rules! Depcrate_instructionis_write_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_write_instruction"}
// Dependencies: {}
pub fn is_write_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 0 == instruction_data [0] }
};
}
