// Generated macro for impl_1435 (impl)
macro_rules! Depcrate_stream_try_stream_try_take_whileimpl_1435 {
() => {
// Module: crate::stream::try_stream::try_take_while
// Provides: {"impl_1435"}
// Dependencies: {}
impl < St , Fut , F > TryTakeWhile < St , Fut , F > where St : TryStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_taking : false } } delegate_access_inner ! (stream , St , ()) ; }
};
}
