// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
# [cfg (unix)] impl < T : AsRawFd > AsRawFd for Async < T > { fn as_raw_fd (& self) -> RawFd { self . get_ref () . as_raw_fd () } }
};
}
