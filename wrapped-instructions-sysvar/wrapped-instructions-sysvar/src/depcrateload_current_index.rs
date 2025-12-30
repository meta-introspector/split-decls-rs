// Generated macro for load_current_index (function)
macro_rules! Depcrateload_current_index {
() => {
// Module: crate
// Provides: {"load_current_index"}
// Dependencies: {}
# [doc = " Load the current `Instruction`'s index in the currently executing"] # [doc = " `Transaction`."] # [doc = ""] # [doc = " `data` is the instructions sysvar account data."] # [doc = ""] # [doc = " Unsafe because the sysvar accounts address is not checked; only used"] # [doc = " internally after such a check."] fn load_current_index (data : & [u8]) -> u16 { let mut instr_fixed_data = [0u8 ; 2] ; let len = data . len () ; instr_fixed_data . copy_from_slice (& data [len - 2 .. len]) ; u16 :: from_le_bytes (instr_fixed_data) }
};
}
