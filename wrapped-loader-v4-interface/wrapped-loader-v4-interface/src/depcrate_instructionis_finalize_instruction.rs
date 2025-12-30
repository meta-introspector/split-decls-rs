// Generated macro for is_finalize_instruction (function)
macro_rules! Depcrate_instructionis_finalize_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_finalize_instruction"}
// Dependencies: {}
pub fn is_finalize_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 6 == instruction_data [0] }
};
}
