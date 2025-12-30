// Generated macro for invalid_syscall (function)
macro_rules! Depcrate_syscalls_tableinvalid_syscall {
() => {
// Module: crate::syscalls::table
// Provides: {"invalid_syscall"}
// Dependencies: {}
extern "C" fn invalid_syscall (sys_no : u64) -> ! { error ! ("Invalid syscall {sys_no}") ; sys_exit (1) ; }
};
}
