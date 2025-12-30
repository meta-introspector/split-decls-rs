// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (windows)] impl < T : AsSocket > AsSocket for Async < T > { fn as_socket (& self) -> BorrowedSocket < '_ > { self . get_ref () . as_socket () } }
};
}
