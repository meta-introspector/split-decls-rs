// Generated macro for syscall3 (function)
macro_rules! Depcrate_syscall_x86_64syscall3 {
() => {
// Module: crate::syscall::x86_64
// Provides: {"syscall3"}
// Dependencies: {}
# [allow (dead_code)] # [inline] pub (crate) fn syscall3 (arg0 : u64 , arg1 : u64 , arg2 : u64 , arg3 : u64) -> u64 { let ret : u64 ; unsafe { asm ! ("syscall" , inlateout ("rax") arg0 => ret , in ("rdi") arg1 , in ("rsi") arg2 , in ("rdx") arg3 , lateout ("rcx") _ , lateout ("r11") _ , options (preserves_flags , nostack)) ; } ret }
};
}
