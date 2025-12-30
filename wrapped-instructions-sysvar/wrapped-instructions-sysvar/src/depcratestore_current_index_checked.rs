// Generated macro for store_current_index_checked (function)
macro_rules! Depcratestore_current_index_checked {
() => {
// Module: crate
// Provides: {"store_current_index_checked"}
// Dependencies: {}
# [doc = " Store the current `Instruction`'s index in the instructions sysvar data."] pub fn store_current_index_checked (data : & mut [u8] , instruction_index : u16 ,) -> Result < () , InstructionError > { if data . len () < 2 { return Err (InstructionError :: AccountDataTooSmall) ; } let last_index = data . len () - 2 ; data [last_index .. last_index + 2] . copy_from_slice (& instruction_index . to_le_bytes ()) ; Ok (()) }
};
}
