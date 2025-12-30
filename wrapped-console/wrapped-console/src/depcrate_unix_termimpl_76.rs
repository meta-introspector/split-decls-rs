// Generated macro for impl_76 (impl)
macro_rules! Depcrate_unix_termimpl_76 {
() => {
// Module: crate::unix_term
// Provides: {"impl_76"}
// Dependencies: {}
impl AsRawFd for Input < fs :: File > { fn as_raw_fd (& self) -> RawFd { match self { Self :: Stdin (s) => s . as_raw_fd () , Self :: File (f) => f . as_raw_fd () , } } }
};
}
