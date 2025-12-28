macro_rules! deps {
    () => {
        PropagateBaseStreamError!();
        Sink!();
    };
}

macro_rules! macro_643 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method."] TryFlattenUnordered < St > (FlattenUnorderedWithFlowController < NestedTryStreamIntoEitherTryStream < St >, PropagateBaseStreamError < St >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| stream : St , limit : impl Into < Option < usize >>| FlattenUnorderedWithFlowController :: new (NestedTryStreamIntoEitherTryStream :: new (stream) , limit . into ())] where St : TryStream , St :: Ok : TryStream , St :: Ok : Unpin , < St :: Ok as TryStream >:: Error : From < St :: Error >) ;
    };
}

macro_643!();