// Generated macro for impl_1315 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1315 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1315"}
// Dependencies: {}
impl < St > NestedTryStreamIntoEitherTryStream < St > where St : TryStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn new (stream : St) -> Self { Self { stream } } delegate_access_inner ! (stream , St , ()) ; }
};
}
