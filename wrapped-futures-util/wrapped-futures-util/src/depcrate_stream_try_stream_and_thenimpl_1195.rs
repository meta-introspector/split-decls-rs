// Generated macro for impl_1195 (impl)
macro_rules! Depcrate_stream_try_stream_and_thenimpl_1195 {
() => {
// Module: crate::stream::try_stream::and_then
// Provides: {"impl_1195"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for AndThen < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (St :: Ok) -> Fut , Fut : TryFuture < Error = St :: Error > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
};
}
