// Generated macro for MAX_CPI_INSTRUCTION_DATA_LEN (const)
macro_rules! Depcrate_syscallsMAX_CPI_INSTRUCTION_DATA_LEN {
() => {
// Module: crate::syscalls
// Provides: {"MAX_CPI_INSTRUCTION_DATA_LEN"}
// Dependencies: {}
# [doc = " Maximum CPI instruction data size. 10 KiB was chosen to ensure that CPI"] # [doc = " instructions are not more limited than transaction instructions if the size"] # [doc = " of transactions is doubled in the future."] pub const MAX_CPI_INSTRUCTION_DATA_LEN : u64 = 10 * 1024 ;
};
}
