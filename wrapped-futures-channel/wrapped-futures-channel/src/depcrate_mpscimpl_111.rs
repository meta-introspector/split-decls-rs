// Generated macro for impl_111 (impl)
macro_rules! Depcrate_mpscimpl_111 {
() => {
// Module: crate::mpsc
// Provides: {"impl_111"}
// Dependencies: {}
impl < St : ? Sized + FusedStream + Unpin > FusedFuture for Recv < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
