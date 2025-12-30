// Generated macro for impl_343 (impl)
macro_rules! Depcrate_stream_stdinimpl_343 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_343"}
// Dependencies: {}
# [cfg (unix)] impl std :: os :: unix :: prelude :: AsRawFd for Stdin { fn as_raw_fd (& self) -> std :: os :: unix :: prelude :: RawFd { self . inner . as_raw_fd () } }
};
}
