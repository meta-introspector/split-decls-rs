// Generated macro for impl_345 (impl)
macro_rules! Depcrate_stream_stdinimpl_345 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_345"}
// Dependencies: {}
# [cfg (all (unix , feature = "polling"))] impl polling :: Source for Stdin { fn raw (& self) -> std :: os :: unix :: prelude :: RawFd { std :: os :: unix :: io :: AsRawFd :: as_raw_fd (self) } }
};
}
