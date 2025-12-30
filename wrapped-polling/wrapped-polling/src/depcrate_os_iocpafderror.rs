// Generated macro for AfdError (struct)
macro_rules! Depcrate_os_iocpAfdError {
() => {
// Module: crate::os::iocp
// Provides: {"AfdError"}
// Dependencies: {}
# [doc = " An error type that wraps around failing to open AFD."] struct AfdError { # [doc = " String description of what happened."] description : & 'static str , # [doc = " The underlying system error."] system : io :: Error , }
};
}
