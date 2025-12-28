macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_710 {
    () => {
        deps!();
        impl < St > TryBufferUnordered < St > where St : TryStream , St :: Ok : TryFuture , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : IntoStream :: new (stream) . fuse () , in_progress_queue : FuturesUnordered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (. .)) ; }
    };
}

impl_710!();