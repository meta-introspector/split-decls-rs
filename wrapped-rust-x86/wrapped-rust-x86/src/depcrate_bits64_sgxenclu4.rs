// Generated macro for enclu4 (function)
macro_rules! Depcrate_bits64_sgxenclu4 {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu4"}
// Dependencies: {}
# [doc = " enclu with four arguments -- consider calling the enclu! macro instead!"] unsafe fn enclu4 (rax : u64 , rbx : u64 , rcx : u64 , rdx : u64) -> (u32 , u64) { let eax : u32 ; let out_rcx : u64 ; asm ! ("pushq %rbx; movq %rsi, %rbx; enclu; popq %rbx" , lateout ("eax") eax , lateout ("rcx") out_rcx , in ("rax") rax , in ("rsi") rbx , in ("rcx") rcx , in ("rdx") rdx , options (att_syntax) ,) ; (eax , out_rcx) }
};
}
