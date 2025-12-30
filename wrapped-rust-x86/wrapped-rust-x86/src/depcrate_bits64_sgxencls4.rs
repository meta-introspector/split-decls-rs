// Generated macro for encls4 (function)
macro_rules! Depcrate_bits64_sgxencls4 {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls4"}
// Dependencies: {}
# [doc = " encls with four arguments -- consider calling the encls! macro instead!"] unsafe fn encls4 (rax : u64 , rbx : u64 , rcx : u64 , rdx : u64) -> (u32 , u64) { let eax : u32 ; let out_rbx : u64 ; asm ! ("pushq %rbx; movq %rsi, %rbx; encls; movq %rbx, %rsi; popq %rbx" , lateout ("eax") eax , lateout ("rsi") out_rbx , in ("rax") rax , in ("rsi") rbx , in ("rcx") rcx , in ("rdx") rdx , options (att_syntax) ,) ; (eax , out_rbx) }
};
}
