// Generated macro for impl_1326 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1326 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1326"}
// Dependencies: {}
impl < St > FusedStream for NestedTryStreamIntoEitherTryStream < St > where St : TryStream + FusedStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
