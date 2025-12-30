// Generated macro for macro_1313 (macro)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedmacro_1313 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"macro_1313"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method."] TryFlattenUnordered < St > (FlattenUnorderedWithFlowController < NestedTryStreamIntoEitherTryStream < St >, PropagateBaseStreamError < St >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| stream : St , limit : impl Into < Option < usize >>| FlattenUnorderedWithFlowController :: new (NestedTryStreamIntoEitherTryStream :: new (stream) , limit . into ())] where St : TryStream , St :: Ok : TryStream , St :: Ok : Unpin , < St :: Ok as TryStream >:: Error : From < St :: Error >) ;
};
}
