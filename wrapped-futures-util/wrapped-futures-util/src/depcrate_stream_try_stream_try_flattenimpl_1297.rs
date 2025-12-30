// Generated macro for impl_1297 (impl)
macro_rules! Depcrate_stream_try_stream_try_flattenimpl_1297 {
() => {
// Module: crate::stream::try_stream::try_flatten
// Provides: {"impl_1297"}
// Dependencies: {}
impl < St > FusedStream for TryFlatten < St > where St : TryStream + FusedStream , St :: Ok : TryStream , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn is_terminated (& self) -> bool { self . next . is_none () && self . stream . is_terminated () } }
};
}
