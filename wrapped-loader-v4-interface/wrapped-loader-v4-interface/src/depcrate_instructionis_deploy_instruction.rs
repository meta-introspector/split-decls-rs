// Generated macro for is_deploy_instruction (function)
macro_rules! Depcrate_instructionis_deploy_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_deploy_instruction"}
// Dependencies: {}
pub fn is_deploy_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 3 == instruction_data [0] }
};
}
