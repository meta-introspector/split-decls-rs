// Generated macro for impl_29 (impl)
macro_rules! Depcrate_streamimpl_29 {
() => {
// Module: crate::stream
// Provides: {"impl_29"}
// Dependencies: {}
impl < P > FusedStream for Pin < P > where P : DerefMut + Unpin , P :: Target : FusedStream , { fn is_terminated (& self) -> bool { < P :: Target as FusedStream > :: is_terminated (& * * self) } }
};
}
