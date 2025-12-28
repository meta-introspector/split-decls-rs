macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl < St > BufferUnordered < St > where St : Stream , St :: Item : Future , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : super :: Fuse :: new (stream) , in_progress_queue : FuturesUnordered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (.)) ; }
    };
}

impl_502!()