// Generated macro for is_transfer_authority_instruction (function)
macro_rules! Depcrate_instructionis_transfer_authority_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_transfer_authority_instruction"}
// Dependencies: {}
pub fn is_transfer_authority_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 5 == instruction_data [0] }
};
}
