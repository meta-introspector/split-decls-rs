// Generated macro for impl_954 (impl)
macro_rules! Depcrate_stream_stream_try_foldimpl_954 {
() => {
// Module: crate::stream::stream::try_fold
// Provides: {"impl_954"}
// Dependencies: {}
impl < St , Fut , T , F > TryFold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : TryFuture < Ok = T > , { pub (super) fn new (stream : St , f : F , t : T) -> Self { Self { stream , f , accum : Some (t) , future : None } } }
};
}
