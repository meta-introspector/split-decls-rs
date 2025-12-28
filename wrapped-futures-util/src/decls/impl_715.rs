macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl < St > TryBuffered < St > where St : TryStream , St :: Ok : TryFuture , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : IntoStream :: new (stream) . fuse () , in_progress_queue : FuturesOrdered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (. .)) ; }
    };
}

impl_715!();