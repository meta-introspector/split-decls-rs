// Generated macro for impl_10 (impl)
macro_rules! Depcrate_async_streamimpl_10 {
() => {
// Module: crate::async_stream
// Provides: {"impl_10"}
// Dependencies: {}
impl < T , U > FusedStream for AsyncStream < T , U > where U : Future < Output = () > , { fn is_terminated (& self) -> bool { self . done } }
};
}
