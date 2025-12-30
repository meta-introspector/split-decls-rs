// Generated macro for impl_1339 (impl)
macro_rules! Depcrate_stream_try_stream_try_collectimpl_1339 {
() => {
// Module: crate::stream::try_stream::try_collect
// Provides: {"impl_1339"}
// Dependencies: {}
impl < St , C > FusedFuture for TryCollect < St , C > where St : TryStream + FusedStream , C : Default + Extend < St :: Ok > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
