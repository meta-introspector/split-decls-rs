// Generated macro for impl_1296 (impl)
macro_rules! Depcrate_stream_try_stream_try_flattenimpl_1296 {
() => {
// Module: crate::stream::try_stream::try_flatten
// Provides: {"impl_1296"}
// Dependencies: {}
impl < St > TryFlatten < St > where St : TryStream , St :: Ok : TryStream , < St :: Ok as TryStream > :: Error : From < St :: Error > , { pub (super) fn new (stream : St) -> Self { Self { stream , next : None } } delegate_access_inner ! (stream , St , ()) ; }
};
}
