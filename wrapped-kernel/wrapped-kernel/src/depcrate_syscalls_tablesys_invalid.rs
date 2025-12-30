// Generated macro for sys_invalid (function)
macro_rules! Depcrate_syscalls_tablesys_invalid {
() => {
// Module: crate::syscalls::table
// Provides: {"sys_invalid"}
// Dependencies: {}
# [allow (unused_assignments)] # [unsafe (no_mangle)] # [unsafe (naked)] pub (crate) unsafe extern "C" fn sys_invalid () { naked_asm ! ("mov rdi, rax" , "call {}" , sym invalid_syscall ,) ; }
};
}
