// Generated macro for syscall0 (function)
macro_rules! Depcrate_syscall_x86_64syscall0 {
() => {
// Module: crate::syscall::x86_64
// Provides: {"syscall0"}
// Dependencies: {}
# [allow (dead_code)] # [inline] pub (crate) fn syscall0 (arg0 : u64) -> u64 { let ret : u64 ; unsafe { asm ! ("syscall" , inlateout ("rax") arg0 => ret , lateout ("rcx") _ , lateout ("r11") _ , options (preserves_flags , nostack)) ; } ret }
};
}
