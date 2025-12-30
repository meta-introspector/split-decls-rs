// Generated macro for is_migrate_instruction (function)
macro_rules! Depcrate_instructionis_migrate_instruction {
() => {
// Module: crate::instruction
// Provides: {"is_migrate_instruction"}
// Dependencies: {}
pub fn is_migrate_instruction (instruction_data : & [u8]) -> bool { ! instruction_data . is_empty () && 8 == instruction_data [0] }
};
}
