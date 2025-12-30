// Generated macro for impl_21 (impl)
macro_rules! Depcrate_uniximpl_21 {
() => {
// Module: crate::unix
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (unix)] impl AsFd for UnixListener { fn as_fd (& self) -> BorrowedFd < '_ > { self . inner . get_ref () . as_fd () } }
};
}
