// Generated macro for MAX_CPI_INSTRUCTION_ACCOUNTS (const)
macro_rules! Depcrate_syscallsMAX_CPI_INSTRUCTION_ACCOUNTS {
() => {
// Module: crate::syscalls
// Provides: {"MAX_CPI_INSTRUCTION_ACCOUNTS"}
// Dependencies: {}
# [doc = " Maximum CPI instruction accounts. 255 was chosen to ensure that instruction"] # [doc = " accounts are always within the maximum instruction account limit for SBF"] # [doc = " program instructions."] pub const MAX_CPI_INSTRUCTION_ACCOUNTS : u8 = u8 :: MAX ;
};
}
