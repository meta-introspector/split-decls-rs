// Generated macro for impl_77 (impl)
macro_rules! Depcrate_unix_termimpl_77 {
() => {
// Module: crate::unix_term
// Provides: {"impl_77"}
// Dependencies: {}
impl AsRawFd for Input < BufReader < fs :: File > > { fn as_raw_fd (& self) -> RawFd { match self { Self :: Stdin (s) => s . as_raw_fd () , Self :: File (f) => f . get_ref () . as_raw_fd () , } } }
};
}
