// Generated macro for sol_get_processed_sibling_instruction (function)
macro_rules! Depcrate_syscallssol_get_processed_sibling_instruction {
() => {
// Module: crate::syscalls
// Provides: {"sol_get_processed_sibling_instruction"}
// Dependencies: {}
# [cfg (target_os = "solana")] # [deprecated (since = "3.1.0" , note = "Use `solana_define_syscall::definitions::get_processed_sibling_instruction` instead")] pub unsafe fn sol_get_processed_sibling_instruction (index : u64 , meta : * mut ProcessedSiblingInstruction , program_id : * mut Pubkey , data : * mut u8 , accounts : * mut AccountMeta ,) -> u64 { unsafe { solana_define_syscall :: definitions :: sol_get_processed_sibling_instruction (index , meta as * mut u8 , program_id as * mut u8 , data , accounts as * mut u8 ,) } }
};
}
