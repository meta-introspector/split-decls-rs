// Generated macro for init (function)
macro_rules! Depcrate_syscallsinit {
() => {
// Module: crate::syscalls
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { Lazy :: force (& SYS) ; SYS . init () ; init_entropy () ; }
};
}
