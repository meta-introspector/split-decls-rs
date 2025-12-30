// Generated macro for syscall2 (function)
macro_rules! Depcrate_bits64_syscallsyscall2 {
() => {
// Module: crate::bits64::syscall
// Provides: {"syscall2"}
// Dependencies: {}
# [doc = " Invoke a syscall."] # [doc = ""] # [doc = " # Safety"] # [doc = " Throws `#UD` if IA32_EFER.SCE = 0."] # [cfg (target_arch = "x86_64")] # [inline (always)] # [allow (unused_mut)] pub unsafe fn syscall2 (arg0 : u64 , arg1 : u64 , arg2 : u64) -> u64 { let mut ret : u64 ; asm ! ("syscall" , lateout ("rax") ret , in ("rax") arg0 , in ("rdi") arg1 , in ("rsi") arg2 , out ("rcx") _ , out ("r11") _ , options (att_syntax) ,) ; ret }
};
}
