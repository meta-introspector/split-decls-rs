// Generated macro for sol_get_return_data (function)
macro_rules! Depcrate_syscallssol_get_return_data {
() => {
// Module: crate::syscalls
// Provides: {"sol_get_return_data"}
// Dependencies: {}
# [deprecated (since = "3.1.0" , note = "Use `solana_define_syscall::definitions::sol_get_return_data` instead")] pub unsafe fn sol_get_return_data (data : * mut u8 , length : u64 , program_id : * mut Pubkey) -> u64 { solana_define_syscall :: definitions :: sol_get_return_data (data , length , program_id as * mut u8) }
};
}
