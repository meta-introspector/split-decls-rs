// Generated macro for encls3 (function)
macro_rules! Depcrate_bits64_sgxencls3 {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls3"}
// Dependencies: {}
# [doc = " encls with three arguments -- consider calling the encls! macro instead!"] unsafe fn encls3 (rax : u64 , rbx : u64 , rcx : u64) -> (u32 , u64) { let eax : u32 ; let out_rbx : u64 ; asm ! ("pushq %rbx; movq %rsi, %rbx; encls; movq %rbx, %r11; popq %rbx" , lateout ("eax") eax , lateout ("rsi") out_rbx , in ("rax") rax , in ("rsi") rbx , in ("rcx") rcx , options (att_syntax) ,) ; (eax , out_rbx) }
};
}
