// Generated macro for impl_1419 (impl)
macro_rules! Depcrate_stream_try_stream_try_skip_whileimpl_1419 {
() => {
// Module: crate::stream::try_stream::try_skip_while
// Provides: {"impl_1419"}
// Dependencies: {}
impl < St , Fut , F > TrySkipWhile < St , Fut , F > where St : TryStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_skipping : false } } delegate_access_inner ! (stream , St , ()) ; }
};
}
