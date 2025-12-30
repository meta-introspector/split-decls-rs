// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_stream_try_stream_try_filterimpl_1254 {
() => {
// Module: crate::stream::try_stream::try_filter
// Provides: {"impl_1254"}
// Dependencies: {}
impl < St , Fut , F > TryFilter < St , Fut , F > where St : TryStream , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None } } delegate_access_inner ! (stream , St , ()) ; }
};
}
