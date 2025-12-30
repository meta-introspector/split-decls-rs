// Generated macro for syscall (macro)
macro_rules! Depcrate_syscall_x86_64syscall {
() => {
// Module: crate::syscall::x86_64
// Provides: {"syscall"}
// Dependencies: {}
# [doc = " This macro can be used to call system functions from user-space"] # [macro_export] macro_rules ! syscall { ($ arg0 : expr) => { $ crate :: syscall :: x86_64 :: syscall0 ($ arg0 as u64) } ; ($ arg0 : expr , $ arg1 : expr) => { $ crate :: syscall :: x86_64 :: syscall1 ($ arg0 as u64 , $ arg1 as u64) } ; ($ arg0 : expr , $ arg1 : expr , $ arg2 : expr) => { $ crate :: syscall :: x86_64 :: syscall2 ($ arg0 as u64 , $ arg1 as u64 , $ arg2 as u64) } ; ($ arg0 : expr , $ arg1 : expr , $ arg2 : expr , $ arg3 : expr) => { $ crate :: syscall :: x86_64 :: syscall3 ($ arg0 as u64 , $ arg1 as u64 , $ arg2 as u64 , $ arg3 as u64) } ; ($ arg0 : expr , $ arg1 : expr , $ arg2 : expr , $ arg3 : expr , $ arg4 : expr) => { $ crate :: syscall :: x86_64 :: syscall4 ($ arg0 as u64 , $ arg1 as u64 , $ arg2 as u64 , $ arg3 as u64 , $ arg4 as u64 ,) } ; ($ arg0 : expr , $ arg1 : expr , $ arg2 : expr , $ arg3 : expr , $ arg4 : expr , $ arg5 : expr) => { $ crate :: syscall :: x86_64 :: syscall5 ($ arg0 as u64 , $ arg1 as u64 , $ arg2 as u64 , $ arg3 as u64 , $ arg4 as u64 , $ arg5 as u64 ,) } ; ($ arg0 : expr , $ arg1 : expr , $ arg2 : expr , $ arg3 : expr , $ arg4 : expr , $ arg5 : expr , $ arg6 : expr) => { $ crate :: syscall :: x86_64 :: syscall6 ($ arg0 as u64 , $ arg1 as u64 , $ arg2 as u64 , $ arg3 as u64 , $ arg4 as u64 , $ arg5 as u64 , $ arg6 as u64 ,) } ; }
};
}
