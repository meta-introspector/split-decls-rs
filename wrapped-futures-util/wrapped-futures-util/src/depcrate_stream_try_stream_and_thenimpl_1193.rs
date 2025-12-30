// Generated macro for impl_1193 (impl)
macro_rules! Depcrate_stream_try_stream_and_thenimpl_1193 {
() => {
// Module: crate::stream::try_stream::and_then
// Provides: {"impl_1193"}
// Dependencies: {}
impl < St , Fut , F > AndThen < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : TryFuture < Error = St :: Error > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
};
}
