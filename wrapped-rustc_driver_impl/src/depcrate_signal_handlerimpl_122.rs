// Generated macro for impl_122 (impl)
macro_rules! Depcrate_signal_handlerimpl_122 {
() => {
// Module: crate::signal_handler
// Provides: {"impl_122"}
// Dependencies: {}
impl fmt :: Write for RawStderr { fn write_str (& mut self , s : & str) -> Result < () , fmt :: Error > { let ret = unsafe { libc :: write (libc :: STDERR_FILENO , s . as_ptr () . cast () , s . len ()) } ; if ret == - 1 { Err (fmt :: Error) } else { Ok (()) } } }
};
}
