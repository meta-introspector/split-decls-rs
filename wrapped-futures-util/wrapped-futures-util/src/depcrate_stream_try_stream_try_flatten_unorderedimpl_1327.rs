// Generated macro for impl_1327 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1327 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1327"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < St , Item > Sink < Item > for NestedTryStreamIntoEitherTryStream < St > where St : TryStream + Sink < Item > , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < < St as TryStream > :: Error > , { type Error = < St as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
};
}
