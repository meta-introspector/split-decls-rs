// Generated macro for encls2 (function)
macro_rules! Depcrate_bits64_sgxencls2 {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls2"}
// Dependencies: {}
# [doc = " encls with two arguments -- consider calling the encls! macro instead!"] unsafe fn encls2 (rax : u64 , rbx : u64) -> (u32 , u64) { let eax : u32 ; let out_rbx : u64 ; asm ! ("pushq %rbx; movq %rsi, %rbx; encls; movq %rbx, %rsi; popq %rbx" , lateout ("eax") eax , lateout ("rsi") out_rbx , in ("rax") rax , in ("rsi") rbx , options (att_syntax) ,) ; (eax , out_rbx) }
};
}
