// Generated macro for is_set_program_length_instruction (function)
macro_rules! Depcrate_instructionis_set_program_length_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_set_program_length_instruction"}
// Dependencies: {}
pub fn is_set_program_length_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 2 == instruction_data [0] }
};
}
