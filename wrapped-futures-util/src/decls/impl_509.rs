macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < St > Buffered < St > where St : Stream , St :: Item : Future , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : super :: Fuse :: new (stream) , in_progress_queue : FuturesOrdered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (.)) ; }
    };
}

impl_509!()