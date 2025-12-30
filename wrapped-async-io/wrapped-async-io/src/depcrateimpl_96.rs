// Generated macro for impl_96 (impl)
macro_rules! Depcrateimpl_96 {
() => {
// Module: crate
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (unix)] impl < T : AsFd > AsFd for Async < T > { fn as_fd (& self) -> BorrowedFd < '_ > { self . get_ref () . as_fd () } }
};
}
