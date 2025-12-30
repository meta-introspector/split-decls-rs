// Generated macro for impl_12 (impl)
macro_rules! Depcrate_futureimpl_12 {
() => {
// Module: crate::future
// Provides: {"impl_12"}
// Dependencies: {}
impl < P > FusedFuture for Pin < P > where P : DerefMut + Unpin , P :: Target : FusedFuture , { fn is_terminated (& self) -> bool { < P :: Target as FusedFuture > :: is_terminated (& * * self) } }
};
}
