// Generated macro for syscall2 (function)
macro_rules! Depcrate_syscall_x86_64syscall2 {
() => {
// Module: crate::syscall::x86_64
// Provides: {"syscall2"}
// Dependencies: {}
# [allow (dead_code)] # [inline] pub (crate) fn syscall2 (arg0 : u64 , arg1 : u64 , arg2 : u64) -> u64 { let ret : u64 ; unsafe { asm ! ("syscall" , inlateout ("rax") arg0 => ret , in ("rdi") arg1 , in ("rsi") arg2 , lateout ("rcx") _ , lateout ("r11") _ , options (preserves_flags , nostack)) ; } ret }
};
}
