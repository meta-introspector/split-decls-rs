// Generated macro for impl_941 (impl)
macro_rules! Depcrate_stream_stream_try_for_eachimpl_941 {
() => {
// Module: crate::stream::stream::try_for_each
// Provides: {"impl_941"}
// Dependencies: {}
impl < St , Fut , F > TryForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : TryFuture < Ok = () > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , future : None } } }
};
}
