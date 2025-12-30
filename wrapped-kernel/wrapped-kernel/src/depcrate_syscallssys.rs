// Generated macro for SYS (static)
macro_rules! Depcrate_syscallsSYS {
() => {
// Module: crate::syscalls
// Provides: {"SYS"}
// Dependencies: {}
pub (crate) static SYS : Lazy < & 'static dyn SyscallInterface > = Lazy :: new (| | { if env :: is_uhyve () { & self :: interfaces :: Uhyve } else { & self :: interfaces :: Generic } }) ;
};
}
