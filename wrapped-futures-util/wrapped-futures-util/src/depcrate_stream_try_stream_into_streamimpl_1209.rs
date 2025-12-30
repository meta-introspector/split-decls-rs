// Generated macro for impl_1209 (impl)
macro_rules! Depcrate_stream_try_stream_into_streamimpl_1209 {
() => {
// Module: crate::stream::try_stream::into_stream
// Provides: {"impl_1209"}
// Dependencies: {}
impl < St : TryStream + FusedStream > FusedStream for IntoStream < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
