// Generated macro for impl_554 (impl)
macro_rules! Depcrate_stream_stream_collectimpl_554 {
() => {
// Module: crate::stream::stream::collect
// Provides: {"impl_554"}
// Dependencies: {}
impl < St , C > FusedFuture for Collect < St , C > where St : FusedStream , C : Default + Extend < St :: Item > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
