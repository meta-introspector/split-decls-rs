// Generated macro for syscall0 (function)
macro_rules! Depcrate_bits64_syscallsyscall0 {
() => {
// Module: crate::bits64::syscall
// Provides: {"syscall0"}
// Dependencies: {}
# [doc = " Invoke a syscall."] # [doc = ""] # [doc = " # Safety"] # [doc = " Throws `#UD` if IA32_EFER.SCE = 0."] # [cfg (target_arch = "x86_64")] # [inline (always)] # [allow (unused_mut)] pub unsafe fn syscall0 (arg0 : u64) -> u64 { let mut ret : u64 ; asm ! ("syscall" , lateout ("rax") ret , in ("rax") arg0 , options (att_syntax)) ; ret }
};
}
