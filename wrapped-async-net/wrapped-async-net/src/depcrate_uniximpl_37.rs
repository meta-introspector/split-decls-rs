// Generated macro for impl_37 (impl)
macro_rules! Depcrate_uniximpl_37 {
() => {
// Module: crate::unix
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (unix)] impl AsFd for UnixStream { fn as_fd (& self) -> BorrowedFd < '_ > { self . inner . get_ref () . as_fd () } }
};
}
