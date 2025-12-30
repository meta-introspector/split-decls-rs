// Generated macro for is_retract_instruction (function)
macro_rules! Depcrate_instructionis_retract_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_retract_instruction"}
// Dependencies: {}
pub fn is_retract_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 4 == instruction_data [0] }
};
}
