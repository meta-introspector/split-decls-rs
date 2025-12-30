// Generated macro for enclu3 (function)
macro_rules! Depcrate_bits64_sgxenclu3 {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu3"}
// Dependencies: {}
# [doc = " enclu with three arguments -- consider calling the enclu! macro instead!"] unsafe fn enclu3 (rax : u64 , rbx : u64 , rcx : u64) -> (u32 , u64) { let eax : u32 ; let out_rcx : u64 ; asm ! ("pushq %rbx; movq %rsi, %rbx; enclu; popq %rbx" , lateout ("eax") eax , lateout ("rcx") out_rcx , in ("rax") rax , in ("rsi") rbx , in ("rcx") rcx , options (att_syntax) ,) ; (eax , out_rcx) }
};
}
