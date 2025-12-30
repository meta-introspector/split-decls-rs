// Generated macro for impl_28 (impl)
macro_rules! Depcrate_streamimpl_28 {
() => {
// Module: crate::stream
// Provides: {"impl_28"}
// Dependencies: {}
impl < F : ? Sized + FusedStream + Unpin > FusedStream for & mut F { fn is_terminated (& self) -> bool { < F as FusedStream > :: is_terminated (& * * self) } }
};
}
