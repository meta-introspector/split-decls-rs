// Generated macro for MYPID (static)
macro_rules! Depcrate_syscallMYPID {
() => {
// Module: crate::syscall
// Provides: {"MYPID"}
// Dependencies: {}
static MYPID : Lazy < RawSpinlock , u32 > = Lazy :: new (| | syscall ! (SyscallNo :: Getpid) . try_into () . unwrap ()) ;
};
}
